use symbols::symbol_name;
use statements::{statement, Statement};
use memory::{regions, Region};
use sections::{sections, Section};

#[derive(Debug, PartialEq)]
pub enum Command {
    Include(String),
    Memory(Vec<Region>),
    Sections(Vec<Section>),
    Statement(Statement),
}

named!(cmd_memory<&str, Command>, wsc!(do_parse!(
    tag_s!("MEMORY")
    >>
    tag_s!("{")
    >>
    regs: regions
    >>
    tag_s!("}")
    >>
    (Command::Memory(regs))
)));

named!(cmd_sections<&str, Command>, wsc!(do_parse!(
    tag_s!("SECTIONS")
    >>
    tag_s!("{")
    >>
    sections: sections
    >>
    tag_s!("}")
    >>
    (Command::Sections(sections))
)));

named!(cmd_statement<&str, Command>, map!(
    statement,
    |x| Command::Statement(x)
));

named!(cmd_include<&str, Command>, wsc!(do_parse!(
    tag_s!("INCLUDE")
    >>
    file: symbol_name
    >>
    (Command::Include(file.into()))
)));

named!(command<&str, Command>, alt_complete!(
    cmd_memory | cmd_sections | cmd_include | cmd_statement
));

named!(pub script<&str, Vec<Command>>, many0!(
    command
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use commands::{command, Command};
    use statements::Statement;
    use expressions::Expression::{Number, Ident};
    use memory::Region;

    #[test]
    fn test_command() {
        assert_eq!(command("OUTPUT_ARCH(msp430)"),
                   IResult::Done("",
                                 Command::Statement(Statement::Command {
                                                        name: String::from("OUTPUT_ARCH"),
                                                        args: vec![Ident(String::from("msp430"))],
                                                    })));

        assert_eq!(command("LONG(0);"),
                   IResult::Done("",
                                 Command::Statement(Statement::Command {
                                                        name: String::from("LONG"),
                                                        args: vec![Number(0)],
                                                    })));

        assert_eq!(command("PROVIDE(. = 0);"),
                   IResult::Done("",
                                 Command::Statement(Statement::Provide {
                                                        symbol: String::from("."),
                                                        expr: Number(0),
                                                    })));

        match command("MEMORY { ABC : o = 1, l = 2 DEF:o=4,l=8}") {
            IResult::Done("", Command::Memory(v)) => {
                assert_eq!(v.len(), 2);
                assert_eq!(v[0],
                           Region {
                               name: "ABC".into(),
                               origin: 1,
                               length: 2,
                           });
                assert_eq!(v[1],
                           Region {
                               name: "DEF".into(),
                               origin: 4,
                               length: 8,
                           });
            }
            _ => assert!(false),
        }

        assert_eq!(command("INCLUDE abc.ld"),
                   IResult::Done("", Command::Include(String::from("abc.ld"))));
    }
}
