use idents::symbol;
use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::map,
    multi::{fold_many0, separated_list0},
    sequence::{delimited, pair},
    IResult,
};
use numbers::number;
use whitespace::opt_space;

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    LogicNot,
    Minus,
    BitwiseNot,
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

fn value_ident(input: &str) -> IResult<&str, Expression> {
    map(symbol, |x: &str| Expression::Ident(x.into()))(input)
}

fn value_number(input: &str) -> IResult<&str, Expression> {
    map(number, |x| Expression::Number(x))(input)
}

fn value_nested(input: &str) -> IResult<&str, Expression> {
    delimited(tag("("), wsc!(expression), tag(")"))(input)
}

fn value_call(input: &str) -> IResult<&str, Expression> {
    let (input, func) = symbol(input)?;
    let (input, _) = wsc!(tag("("))(input)?;
    let (input, args) = separated_list0(wsc!(tag(",")), expression)(input)?;
    let (input, _) = pair(opt_space, tag(")"))(input)?;
    Ok((
        input,
        Expression::Call {
            function: func.into(),
            arguments: args,
        },
    ))
}

pub fn value(input: &str) -> IResult<&str, Expression> {
    alt((value_nested, value_call, value_number, value_ident))(input)
}

fn expr_unary_op(input: &str) -> IResult<&str, Expression> {
    let (input, op) = alt((tag("-"), tag("!"), tag("~")))(input)?;
    let (input, _) = opt_space(input)?;
    let (input, right) = expr_level_1(input)?;
    Ok((
        input,
        Expression::UnaryOp {
            operator: match op {
                "-" => UnaryOperator::Minus,
                "!" => UnaryOperator::LogicNot,
                "~" => UnaryOperator::BitwiseNot,
                _ => panic!("Invalid operator"),
            },
            right: Box::new(right),
        },
    ))
}

fn expr_level_1(input: &str) -> IResult<&str, Expression> {
    alt((expr_unary_op, value))(input)
}

fn expr_level_2(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_1(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(alt((tag("*"), tag("/"), tag("%")))), expr_level_1),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: match new.0 {
                "*" => BinaryOperator::Multiply,
                "/" => BinaryOperator::Divide,
                "%" => BinaryOperator::Remainder,
                _ => panic!("Invalid operator"),
            },
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_3(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_2(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(alt((tag("+"), tag("-")))), expr_level_2),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: match new.0 {
                "+" => BinaryOperator::Plus,
                "-" => BinaryOperator::Minus,
                _ => panic!("Invalid operator"),
            },
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_4(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_3(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(alt((tag("<<"), tag(">>")))), expr_level_3),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: match new.0 {
                "<<" => BinaryOperator::ShiftLeft,
                ">>" => BinaryOperator::ShiftRight,
                _ => panic!("Invalid operator"),
            },
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_5(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_4(input)?;
    let (input, fold) = fold_many0(
        pair(
            wsc!(alt((
                tag("=="),
                tag("!="),
                tag("<="),
                tag(">="),
                tag("<"),
                tag(">")
            ))),
            expr_level_4,
        ),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
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
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_6(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_5(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(tag("&")), expr_level_5),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: BinaryOperator::BitwiseAnd,
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_7(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_6(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(tag("|")), expr_level_6),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: BinaryOperator::BitwiseOr,
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_8(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_7(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(tag("&&")), expr_level_7),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: BinaryOperator::LogicAnd,
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_level_9(input: &str) -> IResult<&str, Expression> {
    let (input, first) = expr_level_8(input)?;
    let (input, fold) = fold_many0(
        pair(wsc!(tag("||")), expr_level_8),
        || first.clone(),
        |prev, new: (&str, Expression)| Expression::BinaryOp {
            left: Box::new(prev),
            operator: BinaryOperator::LogicOr,
            right: Box::new(new.1),
        },
    )(input)?;
    Ok((input, fold))
}

fn expr_ternary_op(input: &str) -> IResult<&str, Expression> {
    let (input, cond) = expr_level_9(input)?;
    let (input, _) = wsc!(tag("?"))(input)?;
    let (input, left) = expression(input)?;
    let (input, _) = wsc!(tag(":"))(input)?;
    let (input, right) = expression(input)?;
    Ok((
        input,
        Expression::TernaryOp {
            condition: Box::new(cond),
            left: Box::new(left),
            right: Box::new(right),
        },
    ))
}

pub fn expression(input: &str) -> IResult<&str, Expression> {
    alt((expr_ternary_op, expr_level_9))(input)
}

#[cfg(test)]
mod tests {
    use expressions::*;

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

        assert_done!(
            expression("A - B"),
            Expression::BinaryOp {
                left: Box::new(Expression::Ident("A".into())),
                operator: BinaryOperator::Minus,
                right: Box::new(Expression::Ident("B".into())),
            }
        );
    }
}
