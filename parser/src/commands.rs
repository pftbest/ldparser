use expressions::Expression;
use expressions::expression;
use idents::{symbol, pattern};
use whitespace::{opt_space, space};

#[derive(Debug, PartialEq)]
pub enum Command {
    Simple { name: String },
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Include { file: String },
}

named!(simple<&str, Command>, do_parse!(
    name: symbol
    >>
    opt_space
    >>
    opt_complete!(tag!(";"))
    >>
    (Command::Simple {
        name: name.into()
    })
));

named!(call<&str, Command>, do_parse!(
    name: symbol
    >>
    wsc!(tag!("("))
    >>
    args: separated_nonempty_list!(
        wsc!(opt_complete!(tag!(","))),
        expression
    )
    >>
    wsc!(tag!(")"))
    >>
    opt_complete!(tag!(";"))
    >>
    (Command::Call {
        name: name.into(),
        arguments: args,
    })
));

named!(include<&str, Command>, do_parse!(
    tag!("INCLUDE")
    >>
    space
    >>
    file: pattern
    >>
    opt_space
    >>
    opt_complete!(tag!(";"))
    >>
    (Command::Include {
        file: file.into()
    })
));

named!(pub command<&str, Command>, alt_complete!(
    include | call | simple
));

mod tests {
    use commands::command;

    #[test]
    fn test_command() {
        assert_done!(command("LONG"));
        assert_done!(command("LONG ;"));
        assert_done!(command("LONG ( 0 ) ;"));
        assert_done!(command("LONG ( 0 )"));
        assert_done!(command("LONG ( 0 1 2 )"));
        assert_done!(command("LONG ( 0, 1 2 )"));
        assert_done!(command("LONG ( 0, 1, 2 )"));

        assert_fail!(command("LONG ( 0, 1, 2, )"));
        assert_fail!(command("LONG ( )"));

        assert_done!(command("INCLUDE abc.h ;"));
        assert_done!(command("INCLUDE\tabc.h"));
    }
}
