use idents::symbol;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{map_res, opt},
    sequence::{delimited, tuple},
    IResult,
};
use whitespace::opt_space;

use crate::{eval::evaluate_expression, expressions::expression};

#[derive(Debug, PartialEq)]
pub struct Region {
    pub name: String,
    pub origin: u64,
    pub length: u64,
}

fn attributes(input: &str) -> IResult<&str, &str> {
    delimited(tag("("), take_until(")"), tag(")"))(input)
}

fn origin(input: &str) -> IResult<&str, &str> {
    alt((tag("ORIGIN"), tag("org"), tag("o")))(input)
}

fn length(input: &str) -> IResult<&str, &str> {
    alt((tag("LENGTH"), tag("len"), tag("l")))(input)
}

pub fn region(input: &str) -> IResult<&str, Region> {
    let (input, name) = symbol(input)?;
    let (input, _) = tuple((
        opt_space,
        opt(attributes),
        wsc!(tag(":")),
        origin,
        wsc!(tag("=")),
    ))(input)?;
    let (input, origin) = map_res(expression, evaluate_expression)(input)?;
    let (input, _) = tuple((wsc!(tag(",")), length, wsc!(tag("="))))(input)?;
    let (input, length) = map_res(expression, evaluate_expression)(input)?;
    Ok((
        input,
        Region {
            name: name.into(),
            origin,
            length,
        },
    ))
}

#[cfg(test)]
mod tests {
    use memory::*;

    #[test]
    fn test_region() {
        assert_done!(
            region("rom (rx)  : ORIGIN = 0, LENGTH = 256K"),
            Region {
                name: "rom".into(),
                origin: 0,
                length: 256 * 1024,
            }
        );
        assert_done!(
            region("ram (!rx) : org = 0x40000000, l = 4M"),
            Region {
                name: "ram".into(),
                origin: 0x40000000,
                length: 4 * 1024 * 1024,
            }
        );
    }

    #[test]
    fn test_region_expr() {
        assert_done!(
            region("FLASH : ORIGIN = 0x08000000, LENGTH = 8K"),
            Region {
                name: "FLASH".into(),
                origin: 0x08000000,
                length: 8 * 1024,
            }
        );

        assert_done!(
            region("RAM : ORIGIN = 0x20000000 + 8K, LENGTH = 640K"),
            Region {
                name: "RAM".into(),
                origin: 0x20000000 + 8 * 1024,
                length: 640 * 1024,
            }
        );

        assert_done!(
            region("RAM : ORIGIN = 0x20000000, LENGTH = 640K - 8K"),
            Region {
                name: "RAM".into(),
                origin: 0x20000000,
                length: 640 * 1024 - 8 * 1024,
            }
        );

        assert_done!(
            region("RAM : ORIGIN = 0x20000000 + 8K - 4K, LENGTH = 640K - 8K + 4K"),
            Region {
                name: "RAM".into(),
                origin: 0x20000000 + 4 * 1024,
                length: 640 * 1024 - 4 * 1024,
            }
        );

        assert_done!(
            region("RAM: ORIGIN = 0x20000000 + 8K - 4K, LENGTH = 640K - 8K + 4K"),
            Region {
                name: "RAM".into(),
                origin: 0x20000000 + 4 * 1024,
                length: 640 * 1024 - 4 * 1024,
            }
        );
    }
}
