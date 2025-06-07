use std::cell::RefCell;

use commands::{command, Command};
use memory::region;
use memory::Region;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::tuple;
use nom::IResult;
use sections::section_command;
use sections::SectionCommand;
use statements::{statement, Statement};
use whitespace::opt_space;

thread_local! {
    pub(crate) static PARSE_STATE: RefCell<ParseState> = RefCell::new(ParseState::default());
}

#[derive(Debug, Default)]
pub struct ParseState {
    pub items: Vec<RootItem>,
}

#[derive(Debug, PartialEq)]
pub enum RootItem {
    Statement(Statement),
    Command(Command),
    Memory { regions: Vec<Region> },
    Sections { list: Vec<SectionCommand> },
}

fn statement_item(input: &str) -> IResult<&str, ()> {
    let (input, stmt) = statement(input)?;
    PARSE_STATE.with_borrow_mut(|s| s.items.push(RootItem::Statement(stmt)));
    Ok((input, ()))
}

fn command_item(input: &str) -> IResult<&str, ()> {
    let (input, cmd) = command(input)?;
    PARSE_STATE.with_borrow_mut(|s| s.items.push(RootItem::Command(cmd)));
    Ok((input, ()))
}

fn memory_item(input: &str) -> IResult<&str, ()> {
    let (mut input, _) = tuple((tag("MEMORY"), wsc!(tag("{"))))(input)?;
    PARSE_STATE.with_borrow_mut(|s| {
        s.items.push(RootItem::Memory {
            regions: Vec::new(),
        })
    });
    loop {
        match wsc!(region)(input) {
            Ok((next_input, region_item)) => {
                PARSE_STATE.with_borrow_mut(|s| {
                    if let Some(RootItem::Memory { regions }) = s.items.last_mut() {
                        regions.push(region_item);
                    }
                });
                input = next_input;
            }
            Err(nom::Err::Error(_)) | Err(nom::Err::Incomplete(_)) => break,
            Err(e) => return Err(e),
        }
    }
    let (input, _) = tag("}")(input)?;
    Ok((input, ()))
}

fn sections_item(input: &str) -> IResult<&str, ()> {
    let (mut input, _) = tuple((tag("SECTIONS"), wsc!(tag("{"))))(input)?;
    PARSE_STATE.with_borrow_mut(|s| s.items.push(RootItem::Sections { list: Vec::new() }));
    loop {
        match wsc!(section_command)(input) {
            Ok((next_input, section_item)) => {
                PARSE_STATE.with_borrow_mut(|s| {
                    if let Some(RootItem::Sections { list }) = s.items.last_mut() {
                        list.push(section_item);
                    }
                });
                input = next_input;
            }
            Err(nom::Err::Error(_)) | Err(nom::Err::Incomplete(_)) => break,
            Err(e) => return Err(e),
        }
    }
    let (input, _) = tag("}")(input)?;
    Ok((input, ()))
}

fn root_item(input: &str) -> IResult<&str, ()> {
    alt((statement_item, memory_item, sections_item, command_item))(input)
}

pub(crate) fn clear_state() {
    // Reset thread-local state
    PARSE_STATE.with_borrow_mut(|state| {
        *state = ParseState::default();
    });
}

pub fn parse(input: &str) -> IResult<&str, Vec<RootItem>> {
    clear_state();

    let mut input = input;
    loop {
        // Try to parse a root_item, skipping optional whitespace before it
        match wsc!(root_item)(input) {
            Ok((next_input, ())) => {
                input = next_input;
            }
            Err(nom::Err::Error(_)) | Err(nom::Err::Incomplete(_)) => {
                // No more root_items found, stop the loop
                break;
            }
            Err(e) => return Err(e),
        }
    }

    // Skip trailing optional whitespace
    let (input, _) = opt_space(input)?;

    let items = PARSE_STATE.with(|s| std::mem::take(&mut *s.borrow_mut()));

    Ok((input, items.items))
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
    fn test_bootloader() {
        let input = include_str!("../tests/bootloader.ld");
        let res = parse(&input);
        assert!(!res.unwrap().1.is_empty());
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
