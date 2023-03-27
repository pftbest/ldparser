use expressions::Expression;
use expressions::expression;
use idents::{symbol, pattern};
use whitespace::{opt_space, space};

#[derive(Debug, PartialEq)]
pub enum InsertOrder {
    Before,
    After,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    //Simple { name: String },
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Include { file: String },
    Insert {
        order: InsertOrder,
        section: String,
    },
}

named!(inset_order<&str, InsertOrder>, alt_complete!(
    map!(tag!("BEFORE"), |_| InsertOrder::Before) |
    map!(tag!("AFTER"), |_| InsertOrder::After)
));

// named!(simple<&str, Command>, do_parse!(
//     name: symbol
//     >>
//     opt_space
//     >>
//     opt_complete!(tag!(";"))
//     >>
//     (Command::Simple {
//         name: name.into()
//     })
// ));

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

named!(insert<&str, Command>, do_parse!(
    tag!("INSERT")
    >>
    order: wsc!(inset_order)
    >>
    section: symbol
    >>
    opt_space
    >>
    opt_complete!(tag!(";"))
    >>
    (Command::Insert {
        order,
        section: section.into(),
    })
));

named!(pub command<&str, Command>, alt_complete!(
    include | call | insert //| simple
));

#[cfg(test)]
mod tests {
    use commands::*;

    #[test]
    fn test_command() {
        assert_done!(command("OUTPUT_ARCH ( 0 ) ;"));
        assert_done!(command("OUTPUT_ARCH ( 0 )"));
        assert_done!(command("OUTPUT_ARCH ( 0 1 2 )"));
        assert_done!(command("OUTPUT_ARCH ( 0, 1 2 )"));
        assert_done!(command("OUTPUT_ARCH ( 0, 1, 2 )"));

        assert_fail!(command("OUTPUT_ARCH ( 0, 1, 2, )"));
        assert_fail!(command("OUTPUT_ARCH ( )"));

        assert_done!(command("INCLUDE abc.h ;"));
        assert_done!(command("INCLUDE\tabc.h"));

        assert_done!(command("INSERT BEFORE .text  ;"));
        assert_done!(command("INSERT  AFTER  .text"));
    }
}
