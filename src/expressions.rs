use symbols::symbol_name;
use numbers::number;

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    Number(u64),
    Nested(Box<Expression>),
    Call {
        function: String,
        argument: Box<Expression>,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    TernaryOp {
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

named!(value_ident<&str, Expression>, map!(
    symbol_name,
    |x: &str| Expression::Ident(x.into())
));

named!(value_number<&str, Expression>, map!(
    number,
    |x| Expression::Number(x)
));

named!(value_nested<&str, Expression>, map!(
    ws!(delimited!(
        tag_s!("("),
        expression,
        tag_s!(")")
    )),
    |x| Expression::Nested(Box::new(x))
));

named!(value_call<&str, Expression>, ws!(do_parse!(
    func: symbol_name
    >>
    tag_s!("(")
    >>
    arg: expression
    >>
    tag_s!(")")
    >>
    (Expression::Call{
        function: func.into(),
        argument: Box::new(arg)
    })
)));

named!(pub value<&str, Expression>, alt_complete!(
    value_call | value_nested | value_number | value_ident
));

named!(binary_operator<&str, &str>, alt_complete!(
    tag_s!("&") | tag_s!("+") | tag_s!("-") | tag_s!("*") | tag_s!("/") | tag_s!("!=")
));

named!(expr_binary_op<&str, Expression>, ws!(do_parse!(
    left: value
    >>
    op: binary_operator
    >>
    right: binary_or_value
    >>
    (Expression::BinaryOp{
        left: Box::new(left),
        operator: op.into(),
        right: Box::new(right),
    })
)));

named!(binary_or_value<&str, Expression>, alt_complete!(
    expr_binary_op | value
));

named!(expr_ternary_op<&str, Expression>, ws!(do_parse!(
    cond: binary_or_value
    >>
    tag_s!("?")
    >>
    left: expression
    >>
    tag_s!(":")
    >>
    right: expression
    >>
    (Expression::TernaryOp{
        condition: Box::new(cond),
        left: Box::new(left),
        right: Box::new(right),
    })
)));

named!(pub expression<&str, Expression>, alt_complete!(
    expr_ternary_op | binary_or_value
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use expressions::expression;
    use expressions::Expression::{Call, Ident, BinaryOp, Number, TernaryOp};

    #[test]
    fn test_expression() {
        assert_eq!(
            expression("a + b ( . * d) - 5K"),
            IResult::Done("",
                BinaryOp{
                    left: Box::new(Ident(String::from("a"))),
                    operator: String::from("+"),
                    right: Box::new(BinaryOp{
                        left: Box::new(Call{
                            function: String::from("b"),
                            argument: Box::new(BinaryOp{
                                left: Box::new(Ident(String::from("."))),
                                operator: String::from("*"),
                                right: Box::new(Ident(String::from("d"))),
                            })
                        }),
                        operator: String::from("-"),
                        right: Box::new(Number(5120))
                    })
                }
            )
        );

        assert_eq!(expression(". != 0 ? 32 / 8 : 1"),
                   IResult::Done("",
                                 TernaryOp {
                                     condition: Box::new(BinaryOp {
                                                             left: Box::new(Ident(String::from("."))),
                                                             operator: String::from("!="),
                                                             right: Box::new(Number(0)),
                                                         }),
                                     left: Box::new(BinaryOp {
                                                        left: Box::new(Number(32)),
                                                        operator: String::from("/"),
                                                        right: Box::new(Number(8)),
                                                    }),
                                     right: Box::new(Number(1)),
                                 }));
    }
}
