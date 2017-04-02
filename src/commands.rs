use symbols::symbol_name;
use expressions::{expression, Expression};
use statements::{statement, Statement};
use memory::{regions, Region};
use sections::{sections, Section};

#[derive(Debug, PartialEq)]
pub enum Command {
    Simple { name: String, args: Vec<Vec<Expression>> },
    Memory(Vec<Region>),
    Statement(Statement),
    Include(String),
    Sections(Vec<Section>),
}

named!(cmd_simple<&str, Command>, wsc!(do_parse!(
    name: symbol_name
    >>
    tag_s!("(")
    >>
    args: separated_nonempty_list!(
        tag_s!(","),
        expression
    )
    >>
    tag_s!(")")
    >>
    (Command::Simple{name: name.into(), args: args})
)));

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
    cmd_memory | cmd_sections | cmd_include | cmd_statement | cmd_simple
));

named!(pub script<&str, Vec<Command>>, many0!(
    command
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use commands::{command, Command};
    use statements::Statement;
    use expressions::{Token, Expression};
    use memory::Region;

    #[test]
    fn test_command() {
        assert_eq!(command("OUTPUT_ARCH(msp430)"),
                   IResult::Done("",
                                 Command::Simple {
                                     name: String::from("OUTPUT_ARCH"),
                                     args: vec![vec![
                                         Expression::Simple(Token::Ident(String::from("msp430")))
                                         ]],
                                 }));

        assert_eq!(command("LONG(0);"),
                   IResult::Done("",
                                 Command::Statement(Statement::Single(Expression::Call {
                                                                          func: String::from("LONG"),
                                                                          args: vec![
                                                 Expression::Simple(Token::Number(0))
                                             ],
                                                                      }))));

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

        let test_sections = r"
SECTIONS
{
  .stack (ORIGIN (RAM) + LENGTH(RAM)) :
  {
    PROVIDE (__stack = .);
    *(.stack)
  } = (0xff)

  .rodata :
  {
    . = ALIGN(2);
    *(.plt)
    *(.rodata .rodata.* .gnu.linkonce.r.* .const .const:*)
    *(.rodata1)
    KEEP (*(.gcc_except_table)) *(.gcc_except_table.*)
    PROVIDE (__preinit_array_start = .);
    KEEP (*(.preinit_array))
    PROVIDE (__preinit_array_end = .);
    PROVIDE (__init_array_start = .);
    KEEP (*(SORT(.init_array.*)))
    KEEP (*(.init_array))
    PROVIDE (__init_array_end = .);
    PROVIDE (__fini_array_start = .);
    KEEP (*(.fini_array))
    KEEP (*(SORT(.fini_array.*)))
    PROVIDE (__fini_array_end = .);
  } > ROM

  /* SGI/MIPS DWARF 2 extensions */
  .debug_weaknames 0 : { *(.debug_weaknames) }
  .debug_funcnames 0 : { *(.debug_funcnames) }
  .debug_typenames 0 : { *(.debug_typenames) }
  .debug_varnames  0 : { *(.debug_varnames) }
  /DISCARD/ : { *(.note.GNU-stack) }
}        ";
        match command(test_sections) {
            IResult::Done("", Command::Sections(v @ _)) => {
                assert_eq!(v.len(), 7);
            }
            r @ _ => panic!("{:?}", r),
        }
    }
}
