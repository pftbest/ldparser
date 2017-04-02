use symbols::symbol_name;
use numbers::number;

#[derive(Debug, PartialEq)]
pub enum Token {
    Ident(String),
    Number(u64),
    Operator(String),
}

named!(tok_number<&str, Token>, map!(
    number,
    |x| Token::Number(x)
));

named!(tok_ident<&str, Token>, map!(
    symbol_name,
    |x: &str| Token::Ident(x.into())
));

named!(operator<&str, &str>, alt_complete!(
    tag_s!("&") | tag_s!("+") | tag_s!("-") | tag_s!("*") | tag_s!("/")
));

named!(tok_operator<&str, Token>, map!(
    operator,
    |x: &str| Token::Operator(x.into())
));

named!(token<&str, Token>, alt_complete!(
    tok_number | tok_operator | tok_ident
));

#[derive(Debug, PartialEq)]
pub enum Expression {
    Simple(Token),
    Nested(Vec<Expression>),
    Call { func: String, args: Vec<Expression> },
}

named!(expr_simple<&str, Expression>, map!(
    token,
    |x| Expression::Simple(x)
));

named!(expr_nested<&str, Expression>, map!(
    delimited!(
        tag_s!("("),
        expression,
        tag_s!(")")
    ),
    |x| Expression::Nested(x)
));

named!(expr_call<&str, Expression>, wsc!(do_parse!(
    func: symbol_name
    >>
    tag_s!("(")
    >>
    args: expression
    >>
    tag_s!(")")
    >>
    (Expression::Call { func: func.into(), args: args})
)));

named!(pub single<&str, Expression>, alt_complete!(
    expr_call | expr_nested | expr_simple
));

named!(pub expression<&str, Vec<Expression>>, wsc!(many0!(
    single
)));

#[cfg(test)]
mod test {
    use nom::IResult;
    use expressions::expression;
    use expressions::{Token, Expression};

    #[test]
    fn test_expression() {
        assert_eq!(expression("a + b ( . * d) - 5K"),
                   IResult::Done("",
                                 vec![Expression::Simple(Token::Ident(String::from("a"))),
                                      Expression::Simple(Token::Operator(String::from("+"))),
                                      Expression::Call {
                                          func: String::from("b"),
                                          args: vec![
                Expression::Simple(Token::Ident(String::from("."))),
                Expression::Simple(Token::Operator(String::from("*"))),
                Expression::Simple(Token::Ident(String::from("d"))),
            ],
                                      },
                                      Expression::Simple(Token::Operator(String::from("-"))),
                                      Expression::Simple(Token::Number(5120))]));
    }
}
