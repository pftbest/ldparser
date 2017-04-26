use statements::{statement, Statement};
use commands::{command, Command};
use memory::Region;
use memory::region;
use sections::SectionCommand;
use sections::section_command;
use whitespace::opt_space;

#[derive(Debug, PartialEq)]
pub enum RootItem {
    Statement(Statement),
    Command(Command),
    Memory { regions: Vec<Region> },
    Sections { list: Vec<SectionCommand> },
}

named!(statement_item<&str, RootItem>, map!(
    statement,
    |stmt| RootItem::Statement(stmt)
));

named!(command_item<&str, RootItem>, map!(
    command,
    |cmd| RootItem::Command(cmd)
));

named!(memory_item<&str, RootItem>, do_parse!(
    tag!("MEMORY")
    >>
    wsc!(tag!("{"))
    >>
    regions: wsc!(many1!(
        region
    ))
    >>
    tag!("}")
    >>
    (RootItem::Memory {
        regions: regions
    })
));

named!(sections_item<&str, RootItem>, do_parse!(
    tag!("SECTIONS")
    >>
    wsc!(tag!("{"))
    >>
    sections: wsc!(many1!(
        section_command
    ))
    >>
    tag!("}")
    >>
    (RootItem::Sections {
        list: sections
    })
));

named!(root_item<&str, RootItem>, alt_complete!(
    statement_item | memory_item | sections_item | command_item
));

named!(pub parse<&str, Vec<RootItem>>, alt_complete!(
    wsc!(many1!(root_item))
    |
    map!(opt_space, |_| vec![])
));

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
