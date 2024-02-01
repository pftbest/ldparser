use idents::symbol;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::opt,
    sequence::{delimited, tuple},
    IResult,
};
use numbers::number;
use whitespace::opt_space;

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
    let (input, org) = number(input)?;
    let (input, _) = tuple((wsc!(tag(",")), length, wsc!(tag("="))))(input)?;
    let (input, len) = number(input)?;
    Ok((
        input,
        Region {
            name: name.into(),
            origin: org,
            length: len,
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
}
