use whitespace::opt_space;
use numbers::number;
use idents::symbol;

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    LogicNot,
    Minus,
    BitwiseNot,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    LogicOr,
    LogicAnd,
    BitwiseOr,
    BitwiseAnd,
    Equals,
    NotEquals,
    Lesser,
    Greater,
    LesserOrEquals,
    GreaterOrEquals,
    ShiftRight,
    ShiftLeft,
    Plus,
    Minus,
    Multiply,
    Divide,
    Remainder,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Ident(String),
    Number(u64),
    Call {
        function: String,
        arguments: Vec<Expression>,
    },
    UnaryOp {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    TernaryOp {
        condition: Box<Expression>,
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

named!(value_ident<&str, Expression>, map!(
    symbol,
    |x: &str| Expression::Ident(x.into())
));

named!(value_number<&str, Expression>, map!(
    number,
    |x| Expression::Number(x)
));

named!(value_nested<&str, Expression>, delimited!(
    tag!("("),
    wsc!(expression),
    tag!(")")
));

named!(value_call<&str, Expression>, do_parse!(
    func: symbol
    >>
    wsc!(tag!("("))
    >>
    args: separated_list!(
        wsc!(tag!(",")),
        expression
    )
    >>
    opt_space
    >>
    tag!(")")
    >>
    (Expression::Call{
        function: func.into(),
        arguments: args
    })
));

named!(pub value<&str, Expression>, alt_complete!(
    value_nested | value_call | value_number | value_ident
));

named!(expr_unary_op<&str, Expression>, do_parse!(
    op: alt_complete!(
        tag!("-") | tag!("!") | tag!("~")
    )
    >>
    opt_space
    >>
    right: expr_level_1
    >>
    (Expression::UnaryOp{
        operator: match op {
            "-" => UnaryOperator::Minus,
            "!" => UnaryOperator::LogicNot,
            "~" => UnaryOperator::BitwiseNot,
            _ => panic!("Invalid operator"),
        },
        right: Box::new(right),
    })
));

named!(expr_level_1<&str, Expression>, alt_complete!(
    expr_unary_op | value
));

named!(expr_level_2<&str, Expression>, do_parse!(
    first: expr_level_1
    >>
    fold: fold_many0!(pair!(
            wsc!(alt_complete!(
                tag!("*") | tag!("/") | tag!("%")
            )),
            expr_level_1
        ),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    "*" => BinaryOperator::Multiply,
                    "/" => BinaryOperator::Divide,
                    "%" => BinaryOperator::Remainder,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_3<&str, Expression>, do_parse!(
    first: expr_level_2
    >>
    fold: fold_many0!(pair!(
            wsc!(alt_complete!(
                tag!("+") | tag!("-")
            )),
            expr_level_2
        ),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    "+" => BinaryOperator::Plus,
                    "-" => BinaryOperator::Minus,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_4<&str, Expression>, do_parse!(
    first: expr_level_3
    >>
    fold: fold_many0!(pair!(
            wsc!(alt_complete!(
                tag!("<<") | tag!(">>")
            )),
            expr_level_3
        ),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    "<<" => BinaryOperator::ShiftLeft,
                    ">>" => BinaryOperator::ShiftRight,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_5<&str, Expression>, do_parse!(
    first: expr_level_4
    >>
    fold: fold_many0!(pair!(
            wsc!(alt_complete!(
                tag!("==") | tag!("!=") | tag!("<=") | tag!(">=") | tag!("<") | tag!(">")
            )),
            expr_level_4
        ),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    "==" => BinaryOperator::Equals,
                    "!=" => BinaryOperator::NotEquals,
                    "<=" => BinaryOperator::LesserOrEquals,
                    ">=" => BinaryOperator::GreaterOrEquals,
                    "<" => BinaryOperator::Lesser,
                    ">" => BinaryOperator::Greater,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_6<&str, Expression>, do_parse!(
    first: expr_level_5
    >>
    fold: fold_many0!(
        pair!(wsc!(tag!("&")), expr_level_5),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::BitwiseAnd,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_7<&str, Expression>, do_parse!(
    first: expr_level_6
    >>
    fold: fold_many0!(
        pair!(wsc!(tag!("|")), expr_level_6),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::BitwiseOr,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_8<&str, Expression>, do_parse!(
    first: expr_level_7
    >>
    fold: fold_many0!(
        pair!(wsc!(tag!("&&")), expr_level_7),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::LogicAnd,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_9<&str, Expression>, do_parse!(
    first: expr_level_8
    >>
    fold: fold_many0!(
        pair!(wsc!(tag!("||")), expr_level_8),
        first,
        |prev, new: (&str, Expression)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::LogicOr,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_ternary_op<&str, Expression>, do_parse!(
    cond: expr_level_9
    >>
    wsc!(tag!("?"))
    >>
    left: expression
    >>
    wsc!(tag!(":"))
    >>
    right: expression
    >>
    (Expression::TernaryOp{
        condition: Box::new(cond),
        left: Box::new(left),
        right: Box::new(right),
    })
));

named!(pub expression<&str, Expression>, alt_complete!(
    expr_ternary_op | expr_level_9
));

#[cfg(test)]
mod tests {
    use expressions::expression;
    use expressions::{Expression, BinaryOperator};

    #[test]
    fn test_ws() {
        let x = "a ( b ( d , ( 0 ) ) , c )";
        assert_done!(expression(x));
        let y = "a(b(d,(0)),c)";
        assert_eq!(expression(x), expression(y));
    }

    #[test]
    fn test_expression() {
        assert_done!(expression("a ( .b ) ? c ( d ) : e"));

        assert_done!(expression("A-B"), Expression::Ident("A-B".into()));

        assert_done!(expression("A - B"),
                     Expression::BinaryOp {
                         left: Box::new(Expression::Ident("A".into())),
                         operator: BinaryOperator::Minus,
                         right: Box::new(Expression::Ident("B".into())),
                     });
    }
}
