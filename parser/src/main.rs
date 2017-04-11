#![feature(test)]
extern crate test;

#[macro_use]
extern crate nom;

use nom::digit;
use nom::alphanumeric;
use std::str::FromStr;
use std::str;

named!(symbol_name, call!(alphanumeric));

named!(number<u64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

// ========================================================================== //

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
    Negate,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    LogicOr,
    LogicAnd,
    Or,
    And,
    Equals,
    NotEquals,
    Greater,
    Less,
    LessOrEquals,
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
pub enum Expression<'a> {
    Ident(&'a str),
    Number(u64),
    Call {
        function: &'a str,
        args: Vec<Expression<'a>>,
    },
    UnaryOp {
        operator: UnaryOperator,
        right: Box<Expression<'a>>,
    },
    BinaryOp {
        left: Box<Expression<'a>>,
        operator: BinaryOperator,
        right: Box<Expression<'a>>,
    },
    TernaryOp {
        condition: Box<Expression<'a>>,
        left: Box<Expression<'a>>,
        right: Box<Expression<'a>>,
    },
}

named!(value_ident<Expression>, map!(
    map_res!(
        symbol_name,
        str::from_utf8
    ),
    |x| Expression::Ident(x)
));

named!(value_number<Expression>, map!(
    number,
    |x| Expression::Number(x)
));

named!(value_nested<Expression>, delimited!(
    tag!("("),
    ws!(expression),
    tag!(")")
));

named!(value_call<Expression>, do_parse!(
    func: map_res!(symbol_name, str::from_utf8)
    >>
    ws!(tag!("("))
    >>
    args: separated_list!(
        ws!(tag!(",")),
        expression
    )
    >>
    ws!(tag!(")"))
    >>
    (Expression::Call{
        function: func,
        args: args
    })
));

named!(pub value<Expression>, alt_complete!(
    value_call | value_nested | value_number | value_ident
));

named!(expr_unary_op<Expression>, do_parse!(
    op: ws!(alt_complete!(
        tag!("-") | tag!("!") | tag!("~")
    ))
    >>
    right: expr_level_1
    >>
    (Expression::UnaryOp{
        operator: match op[0] as char {
            '-' => UnaryOperator::Minus,
            '!' => UnaryOperator::Not,
            '~' => UnaryOperator::Negate,
            _ => panic!("Invalid operator"),
        },
        right: Box::new(right),
    })
));

named!(expr_level_1<Expression>, alt_complete!(
    expr_unary_op | value
));

named!(expr_level_2<Expression>, do_parse!(
    first: expr_level_1
    >>
    fold: fold_many0!(pair!(
            ws!(alt_complete!(
                tag!("*") | tag!("/") | tag!("%")
            )),
            expr_level_1
        ),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0[0] as char {
                    '*' => BinaryOperator::Multiply,
                    '/' => BinaryOperator::Divide,
                    '%' => BinaryOperator::Remainder,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_3<Expression>, do_parse!(
    first: expr_level_2
    >>
    fold: fold_many0!(pair!(
            ws!(alt_complete!(
                tag!("+") | tag!("-")
            )),
            expr_level_2
        ),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0[0] as char {
                    '+' => BinaryOperator::Plus,
                    '-' => BinaryOperator::Minus,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_4<Expression>, do_parse!(
    first: expr_level_3
    >>
    fold: fold_many0!(pair!(
            ws!(alt_complete!(
                tag!("<<") | tag!(">>")
            )),
            expr_level_3
        ),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    b"<<" => BinaryOperator::ShiftLeft,
                    b">>" => BinaryOperator::ShiftRight,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_5<Expression>, do_parse!(
    first: expr_level_4
    >>
    fold: fold_many0!(pair!(
            ws!(alt_complete!(
                tag!("==") | tag!("!=") | tag!("<=") | tag!(">=") | tag!("<") | tag!(">")
            )),
            expr_level_4
        ),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: match new.0 {
                    b"==" => BinaryOperator::Equals,
                    b"!=" => BinaryOperator::NotEquals,
                    b"<=" => BinaryOperator::LessOrEquals,
                    b">=" => BinaryOperator::GreaterOrEquals,
                    b"<" => BinaryOperator::Less,
                    b">" => BinaryOperator::Greater,
                    _ => panic!("Invalid operator"),
                },
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_6<Expression>, do_parse!(
    first: expr_level_5
    >>
    fold: fold_many0!(
        pair!(ws!(tag!("&")), expr_level_5),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::And,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_7<Expression>, do_parse!(
    first: expr_level_6
    >>
    fold: fold_many0!(
        pair!(ws!(tag!("|")), expr_level_6),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
            Expression::BinaryOp {
                left: Box::new(prev),
                operator: BinaryOperator::Or,
                right: Box::new(new.1)
            }
        }
    )
    >>
    (fold)
));

named!(expr_level_8<Expression>, do_parse!(
    first: expr_level_7
    >>
    fold: fold_many0!(
        pair!(ws!(tag!("&&")), expr_level_7),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
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

named!(expr_level_9<Expression>, do_parse!(
    first: expr_level_8
    >>
    fold: fold_many0!(
        pair!(ws!(tag!("||")), expr_level_8),
        first,
        |prev, new: (&'a [u8], Expression<'a>)| {
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

named!(expr_ternary_op<Expression>, do_parse!(
    cond: expr_level_9
    >>
    ws!(tag!("?"))
    >>
    left: expression
    >>
    ws!(tag!(":"))
    >>
    right: expression
    >>
    (Expression::TernaryOp{
        condition: Box::new(cond),
        left: Box::new(left),
        right: Box::new(right),
    })
));

named!(pub expression<Expression>, alt_complete!(
    expr_ternary_op | expr_level_9
));

macro_rules! assert_done {
    ($res:expr) => (
        match $res {
            nom::IResult::Done(b"", v @ _) => {
                println!("ok: {:?}", v);
            }
            r @ _ => panic!("fail: {:?}", r),
        }
    )
}

#[test]
fn test_ws() {
    let x = b"a ( b ( d , ( 0 ) ) , c )";
    assert_done!(expression(x));
    let y = b"a(b(d,(0)),c)";
    assert_eq!(expression(x), expression(y));
}

#[test]
fn test_ternary() {
    let x = b"a ( b ) ? c ( d ) : e";
    assert_done!(expression(x));
}

#[test]
fn test_logic_or() {
    let x = b"a || b";
    assert_done!(expression(x));
}

#[bench]
fn bench_some(b: &mut test::Bencher) {
    b.iter(|| {
               let x = test::black_box(b"a ( b ( d , ( 0 < 3 ) ) , c | d )");
               expression(x)
           })
}

fn main() {
    let x = b"a - ! - b";
    println!("{:?}", expression(x));
}
