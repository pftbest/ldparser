use nom::{
    branch::alt,
    bytes::complete::{tag, take_till, take_until, take_while1},
    character::complete::satisfy,
    combinator::recognize,
    sequence::{delimited, pair},
    IResult,
};

pub fn string(input: &str) -> IResult<&str, &str> {
    delimited(tag("\""), take_until("\""), tag("\""))(input)
}

fn simple(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        satisfy(|c| c.is_alphabetic() || c == '_' || c == '.'),
        take_till(|c: char| !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-')),
    ))(input)
}

pub fn symbol(input: &str) -> IResult<&str, &str> {
    alt((string, simple))(input)
}

fn is_pattern(c: char) -> bool {
    c.is_alphanumeric() || "_.$/\\~=+[]*?-!<>^:".contains(c)
}

fn simple_pattern(input: &str) -> IResult<&str, &str> {
    take_while1(is_pattern)(input)
}

pub fn pattern(input: &str) -> IResult<&str, &str> {
    alt((string, simple_pattern))(input)
}

#[cfg(test)]
mod tests {
    use idents::*;

    #[test]
    fn test_symbol() {
        assert_done!(symbol(".0"), ".0");
        assert_done!(symbol(".text"), ".text");
        assert_done!(symbol("a-b"), "a-b");
        assert_done!(
            symbol("\"spaces are ok, just quote the identifier\""),
            "spaces are ok, just quote the identifier"
        );
    }

    #[test]
    fn test_pattern() {
        assert_done!(pattern("0"), "0");
        assert_done!(pattern(".text"), ".text");
        assert_done!(pattern("hello*.o"), "hello*.o");
        assert_done!(
            pattern("\"spaces are ok, just quote the identifier\""),
            "spaces are ok, just quote the identifier"
        );
        assert_done!(
            pattern("this+is-another*crazy[example]"),
            "this+is-another*crazy[example]"
        );
    }
}
