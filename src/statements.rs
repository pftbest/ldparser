use symbols::symbol_name;
use expressions::{expression, value, Expression};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        symbol: String,
        operator: String,
        expr: Expression,
    },
    Provide { symbol: String, expr: Expression },
    Single(Expression),
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

named!(stmt_single<&str, Statement>, map!(
    terminated!(
        value,
        tag_s!(";")
    ),
    |x| Statement::Single(x)
));

named!(pub statement<&str, Statement>, alt_complete!(
    stmt_assign | stmt_provide | stmt_single
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
    }
}
