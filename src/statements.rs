use symbols::symbol_name;
use expressions::{expression, single, Expression};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Assign {
        symbol: String,
        operator: String,
        expr: Vec<Expression>,
    },
    Provide {
        symbol: String,
        expr: Vec<Expression>,
    },
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
        single,
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
    use expressions::{Token, Expression};

    #[test]
    fn test_statement() {
        assert_eq!(statement(" . = ALIGN ( 10 ) ; "),
                   IResult::Done("",
                                 Statement::Assign {
                                     symbol: String::from("."),
                                     operator: String::from("="),
                                     expr: vec![Expression::Call {
                                                    func: String::from("ALIGN"),
                                                    args: vec![
                                                        Expression::Simple(Token::Number(10))
                                                    ],
                                                }],
                                 }));
        assert_eq!(statement(" PROVIDE ( TEST = . + 1 ) ;"),
                   IResult::Done("",
                                 Statement::Provide {
                                     symbol: String::from("TEST"),
                                     expr: vec![Expression::Simple(Token::Ident(String::from("."))),
                                     Expression::Simple(Token::Operator(String::from("+"))),
                                     Expression::Simple(Token::Number(1))
                                    ],
                                 }));
    }
}
