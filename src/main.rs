#[macro_use]
extern crate nom;
use nom::{multispace, digit, hex_digit, IResult, Needed, ErrorKind};

use std::str::FromStr;

named!(comment<&str, &str>, delimited!(
    tag_s!("/*"),
    take_until_s!("*/"),
    tag_s!("*/")
));

named!(space_or_comment<&str, Vec<&str>>, many0!(
    alt!(multispace | comment)
));

/// Transforms a parser to automatically consume whitespace and comments
/// between each token.
macro_rules! wsc(
    ($i:expr, $($args:tt)*) => ({sep!($i, space_or_comment, $($args)*)})
);

/// Recognizes one or more numerical and alphabetic characters: 0-9a-zA-Z
/// and also allows '.' and '_' characters.
pub fn identifier(input: &str) -> IResult<&str, &str> {
    let input_length = input.len();
    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }
    for (idx, item) in input.char_indices() {
        if !(item.is_alphanumeric() || item == '_' || item == '.') {
            if idx == 0 {
                return IResult::Error(error_position!(ErrorKind::AlphaNumeric, input));
            } else {
                return IResult::Done(&input[idx..], &input[0..idx]);
            }
        }
    }
    IResult::Done(&input[input_length..], input)
}

/// Recognizes one or more numerical and alphabetic characters: 0-9a-zA-Z
/// and also allows '.', '_', and '*' characters.
pub fn pattern(input: &str) -> IResult<&str, &str> {
    let input_length = input.len();
    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }
    for (idx, item) in input.char_indices() {
        if !(item.is_alphanumeric() || item == '_' || item == '.' || item == '*') {
            if idx == 0 {
                return IResult::Error(error_position!(ErrorKind::AlphaNumeric, input));
            } else {
                return IResult::Done(&input[idx..], &input[0..idx]);
            }
        }
    }
    IResult::Done(&input[input_length..], input)
}

named!(number<&str, u64>,
    alt!(
        preceded!(
            tag_no_case_s!("0x"),
            map_res!(
                hex_digit,
                |x| u64::from_str_radix(x, 16)
            )
        )
        |
        map_res!(
            digit,
            FromStr::from_str
        )
    )
);

#[derive(Debug)]
struct Region {
    name: String,
    flags: String,
    origin: u64,
    length: u64,
}

#[derive(Debug)]
enum Expression {
    Call { name: String, args: Vec<Expression> },
    Ident ( String ),
    Value ( u64 ),
    Assignment { lvalue: String, rvalue: Box<Expression> }
    Match { pattern: String, args: Vec<Expression> },
}

#[derive(Debug)]
struct Section {
    name: String,
    exprs: Vec<Expression>
}

#[derive(Debug)]
enum RootItem {
    Command(Expression),
    Memory(Vec<Region>),
    Sections(Vec<Section>),
}

named!(assignment<&str, Expression>, wsc!(
    do_parse!(
        name: identifier
        >>
        tag_s!("=")
        >>
        arg: expression
        >>
        (Expression::Assignment{lvalue: name.into(), rvalue: Box::new(arg)})
    )
));

named!(call<&str, Expression>, wsc!(
    do_parse!(
        name: identifier
        >>
        args: delimited!(
            tag_s!("("),
            separated_list!(tag_s!(","), expression),
            tag_s!(")")
        )
        >>
        (Expression::Call{name: name.into(), args: args})
    )
));

named!(match<&str, Expression>, wsc!(
    do_parse!(
        name: pattern
        >>
        args: delimited!(
            tag_s!("("),
            many1!(expression),
            tag_s!(")")
        )
        >>
        (Expression::Call{pattern: name.into(), args: args})
    )
));

named!(ident<&str, Expression>, wsc!(
    do_parse!(
        name: identifier
        >>
        (Expression::Ident(name.into()))
    ))
);

named!(value<&str, Expression>, wsc!(
    do_parse!(
        num: number
        >>
        (Expression::Value(num))
    ))
);

named!(parens<&str, Expression>, wsc!(
    delimited!(
        tag_s!("("),
        take_until_s!(")"),
        tag_s!(")")
    )
);

named!(expression<&str, Expression>,
    alt!( call | assignment | value | ident)
);

named!(region<&str, Region>, wsc!(
    do_parse!(
        name: identifier
        >>
        flags: opt!(delimited!(
            tag_s!("("),
            take_until_s!(")"),
            tag_s!(")")
        ))
        >>
        tag_s!(":")
        >>
        alt!(
            tag_s!("ORIGIN")
            |
            tag_s!("org")
            |
            tag_s!("o")
        )
        >>
        tag_s!("=")
        >>
        origin: number
        >>
        tag_s!(",")
        >>
        alt!(
            tag_s!("LENGTH")
            |
            tag_s!("len")
            |
            tag_s!("l")
        )
        >>
        tag_s!("=")
        >>
        length: number
        >>
        (Region { name: name.into(), origin: origin, length: length, flags: flags.unwrap_or("").into()})
    )
));

named!(section<&str, Section>, wsc!(
    do_parse!(
        name: identifier
        >>
        attribute: opt!(expression)
        >>
        tag_s!(":")
        >>
        tag_s!("{")
        >>
        exprs: many0!(expression)
        >>
        tag_s!("}")
        >>
        tag_s!(">")
        >>
        dest: identifier
        >>
        (Section { name: name.into(), exprs: exprs })
    )
));

named!(memory_block<&str, RootItem>, wsc!(
    do_parse!(
        tag_s!("MEMORY")
        >>
        regions: delimited!(
            tag_s!("{"),
            many0!(region),
            tag_s!("}")
        )
        >>
        (RootItem::Memory(regions))
    )
));

named!(sections_block<&str, RootItem>, wsc!(
    do_parse!(
        tag_s!("SECTIONS")
        >>
        sections: delimited!(
            tag_s!("{"),
            many1!(section),
            tag_s!("}")
        )
        >>
        (RootItem::Sections(sections))
    )
));

named!(command<&str, RootItem>, wsc!(
    do_parse!(
        expr: call
        >>
        (RootItem::Command(expr))
    )
));

named!(root_item<&str, RootItem>, alt!(
    memory_block
    |
    sections_block
    |
    command
));

named!(script<&str, Vec<RootItem>>, wsc!(many0!(root_item)));

fn main() {
    //let first_line = "abc /* dddd */ ( hello ) ";
    let first_line = include_str!("/home/user/Public/rust_on_msp/ldscripts/msp430g2553.ld");
    let res = script(first_line);

    // let regs = r"MEMORY { SFR      ( rwx )        : ORIGIN = 0x0000, LENGTH = 0x0010
    // /* END=0x0010, size 16 */  RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, si
    // ze 512 */  INFOMEM          : ORIGIN = 0x1000, LENGTH = 0x0100 /* END=0x10FF, size 256 as 4 64-byte segments */
    // }";
    // let res = memory_block(regs);

    //let sect = "SECTIONS\n{\n\n  .bslsignature       : {} > BSLSIGNATURE\n __interrupt_vector_1   : { KEEP (*(__interrupt_vector_1 )) KEEP (*(__interrupt_vector_trapint)) } > VECT1 }";
    //let res = sections_block(sect);

    //let expr = " KEEP (*(__interrupt_vector_1 )) ";
    //let res = expression(expr);

    println!("{:?}", res);
}
