use symbols::symbol_name;
use expressions::{expression, Expression};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        symbol: String,
        operator: String,
        expr: Expression,
    },
    Provide { symbol: String, expr: Expression },
    Command { name: String, args: Expression },
}

named!(assign_operator<&str, &str>, alt_complete!(
    tag_s!("=") | tag_s!("&=") | tag_s!("+=") | tag_s!("-=") | tag_s!("*=") | tag_s!("/=")
));

named!(stmt_assign<&str, Statement>, wsc!(do_parse!(
    symbol: symbol_name
    >>
    op: assign_operator
    >>
    expr: expression
    >>
    tag_s!(";")
    >>
    (Statement::Assign{symbol: symbol.into(), operator: op.into(), expr: expr})
)));

named!(stmt_provide<&str, Statement>, wsc!(do_parse!(
    tag_s!("PROVIDE")
    >>
    tag_s!("(")
    >>
    symbol: symbol_name
    >>
    tag_s!("=")
    >>
    expr: expression
    >>
    tag_s!(")")
    >>
    tag_s!(";")
    >>
    (Statement::Provide{symbol: symbol.into(), expr: expr})
)));

named!(stmt_command<&str, Statement>, wsc!(do_parse!(
    name: symbol_name
    >>
    tag_s!("(")
    >>
    expr: expression
    >>
    tag_s!(")")
    >>
    opt!(complete!(tag_s!(";")))
    >>
    (Statement::Command{name: name.into(), args: expr})
)));

named!(pub statement<&str, Statement>, alt_complete!(
    stmt_assign | stmt_provide | stmt_command
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use statements::{statement, Statement};
    use expressions::Expression::{BinaryOp, Call, Ident, Number};

    #[test]
    fn test_statement() {
        assert_eq!(statement(" . = ALIGN ( 10 ) ; "),
                   IResult::Done("",
                                 Statement::Assign {
                                     symbol: String::from("."),
                                     operator: String::from("="),
                                     expr: Call {
                                         function: String::from("ALIGN"),
                                         argument: Box::new(Number(10)),
                                     },
                                 }));

        assert_eq!(statement(" PROVIDE ( TEST = . + 1 ) ;"),
                   IResult::Done("",
                                 Statement::Provide {
                                     symbol: String::from("TEST"),
                                     expr: BinaryOp {
                                         left: Box::new(Ident(String::from("."))),
                                         operator: String::from("+"),
                                         right: Box::new(Number(1)),
                                     },
                                 }));
        assert_eq!(statement("OUTPUT_ARCH(msp430)"),
                   IResult::Done("",
                                 Statement::Command {
                                     name: String::from("OUTPUT_ARCH"),
                                     args: Ident(String::from("msp430")),
                                 }));
    }
}
