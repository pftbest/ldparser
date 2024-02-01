use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::multispace1,
    combinator::recognize,
    multi::{fold_many0, fold_many1},
    sequence::delimited,
    IResult,
};

pub fn comment(input: &str) -> IResult<&str, &str> {
    delimited(tag("/*"), take_until("*/"), tag("*/"))(input)
}

pub fn space_or_comment(input: &str) -> IResult<&str, &str> {
    alt((multispace1, comment))(input)
}

pub fn space(input: &str) -> IResult<&str, &str> {
    recognize(fold_many1(space_or_comment, || (), |_, _| ()))(input)
}

pub fn opt_space(input: &str) -> IResult<&str, &str> {
    recognize(fold_many0(space_or_comment, || (), |_, _| ()))(input)
}

/// Transforms a parser to automatically consume whitespace and comments
/// between each token.
macro_rules! wsc(
    ($arg:expr) => ({
        use $crate::whitespace::opt_space;
        use nom::sequence::delimited;
        delimited(opt_space, $arg, opt_space)
    });

    ($arg0:expr, $($args:expr),+) => ({
        use $crate::whitespace::opt_space;
        use nom::sequence::delimited;
        use nom::sequence::tuple;
        delimited(opt_space, tuple(($arg0, $($args),*)), opt_space)
    })
);

#[cfg(test)]
mod tests {
    use nom::{
        bytes::complete::{tag, take_while1},
        multi::many0,
        sequence::tuple,
    };
    use whitespace::opt_space;

    fn is_good(c: char) -> bool {
        c.is_alphanumeric() || c == '/' || c == '*'
    }

    #[test]
    fn test_wsc() {
        let mut test_parser = many0(wsc!(take_while1(is_good)));
        let input = "a /* b */ c / * d /**/ e ";
        assert_done!(test_parser(input), vec!["a", "c", "/", "*", "d", "e"]);
    }

    #[test]
    fn test_opt_space() {
        let mut test_parser = tuple((
            tag("("),
            opt_space,
            take_while1(is_good),
            opt_space,
            tag(")"),
        ));

        let input1 = "(  a  )";
        assert_done!(test_parser(input1));

        let input2 = "(a)";
        assert_done!(test_parser(input2));
    }
}
