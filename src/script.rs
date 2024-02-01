use commands::{command, Command};
use memory::region;
use memory::Region;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;
use sections::section_command;
use sections::SectionCommand;
use statements::{statement, Statement};
use whitespace::opt_space;

#[derive(Debug, PartialEq)]
pub enum RootItem {
    Statement(Statement),
    Command(Command),
    Memory { regions: Vec<Region> },
    Sections { list: Vec<SectionCommand> },
}

fn statement_item(input: &str) -> IResult<&str, RootItem> {
    map(statement, |stmt| RootItem::Statement(stmt))(input)
}

fn command_item(input: &str) -> IResult<&str, RootItem> {
    map(command, |cmd| RootItem::Command(cmd))(input)
}

fn memory_item(input: &str) -> IResult<&str, RootItem> {
    let (input, _) = tuple((tag("MEMORY"), wsc!(tag("{"))))(input)?;
    let (input, regions) = many1(wsc!(region))(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, RootItem::Memory { regions: regions }))
}

fn sections_item(input: &str) -> IResult<&str, RootItem> {
    let (input, _) = tuple((tag("SECTIONS"), wsc!(tag("{"))))(input)?;
    let (input, sections) = many1(wsc!(section_command))(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, RootItem::Sections { list: sections }))
}

fn root_item(input: &str) -> IResult<&str, RootItem> {
    alt((statement_item, memory_item, sections_item, command_item))(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<RootItem>> {
    alt((many1(wsc!(root_item)), map(opt_space, |_| vec![])))(input)
}

#[cfg(test)]
mod tests {
    use script::*;
    use std::fs::{self, File};
    use std::io::Read;

    #[test]
    fn test_empty() {
        assert_done_vec!(parse(""), 0);
        assert_done_vec!(parse("                               "), 0);
        assert_done_vec!(parse("      /* hello */              "), 0);
    }

    #[test]
    fn test_parse() {
        for entry in fs::read_dir("tests").unwrap() {
            let path = entry.unwrap().path();
            println!("testing: {:?}", path);
            let mut file = File::open(path).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            assert_done!(parse(&contents));
        }
    }
}
