use expressions::expression;
use expressions::Expression;
use idents::{pattern, symbol};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::IResult;
use whitespace::{opt_space, space};

#[derive(Debug, PartialEq)]
pub enum InsertOrder {
    Before,
    After,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    //Simple { name: String },
    Call {
        name: String,
        arguments: Vec<Expression>,
    },
    Include {
        file: String,
    },
    Insert {
        order: InsertOrder,
        section: String,
    },
}

fn inset_order(input: &str) -> IResult<&str, InsertOrder> {
    alt((
        map(tag("BEFORE"), |_| InsertOrder::Before),
        map(tag("AFTER"), |_| InsertOrder::After),
    ))(input)
}

fn call(input: &str) -> IResult<&str, Command> {
    let (input, name) = symbol(input)?;
    let (input, _) = wsc!(tag("("))(input)?;
    let (input, args) = separated_list1(alt((space, wsc!(tag(",")))), expression)(input)?;
    let (input, _) = pair(wsc!(tag(")")), opt(tag(";")))(input)?;
    Ok((
        input,
        Command::Call {
            name: name.into(),
            arguments: args,
        },
    ))
}

fn include(input: &str) -> IResult<&str, Command> {
    let (input, _) = pair(tag("INCLUDE"), space)(input)?;
    let (input, file) = pattern(input)?;
    let (input, _) = pair(opt_space, opt(tag(";")))(input)?;
    Ok((input, Command::Include { file: file.into() }))
}

fn insert(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("INSERT")(input)?;
    let (input, order) = wsc!(inset_order)(input)?;
    let (input, section) = symbol(input)?;
    let (input, _) = pair(opt_space, opt(tag(";")))(input)?;
    Ok((
        input,
        Command::Insert {
            order,
            section: section.into(),
        },
    ))
}

pub fn command(input: &str) -> IResult<&str, Command> {
    alt((include, call, insert))(input)
}

#[cfg(test)]
mod tests {
    use commands::*;

    #[test]
    fn test_command() {
        assert_done!(command("OUTPUT_ARCH ( 0 ) ;"));
        assert_done!(command("OUTPUT_ARCH ( 0 )"));
        assert_done!(command("OUTPUT_ARCH ( 0 1 2 )"));
        assert_done!(command("OUTPUT_ARCH ( 0, 1 2 )"));
        assert_done!(command("OUTPUT_ARCH ( 0, 1, 2 )"));

        assert_fail!(command("OUTPUT_ARCH ( 0, 1, 2, )"));
        assert_fail!(command("OUTPUT_ARCH ( )"));

        assert_done!(command("INCLUDE abc.h ;"));
        assert_done!(command("INCLUDE\tabc.h"));

        assert_done!(command("INSERT BEFORE .text  ;"));
        assert_done!(command("INSERT  AFTER  .text"));
    }
}
