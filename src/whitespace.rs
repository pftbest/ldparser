use nom::multispace;

named!(pub comment<&str, &str>, delimited!(
    tag_s!("/*"),
    take_until_s!("*/"),
    tag_s!("*/")
));

named!(pub space_or_comment<&str, Vec<&str>>, many0!(
    alt!(multispace | comment)
));

/// Transforms a parser to automatically consume whitespace and comments
/// between each token.
macro_rules! wsc(
    ($i:expr, $($args:tt)*) => ({
        use $crate::whitespace::space_or_comment;
        sep!($i, space_or_comment, $($args)*)
    })
);

#[cfg(test)]
mod test {
    use nom::{IResult, Needed};

    #[test]
    fn test_comment() {
        named!(test_parser<&str, Vec<char>>, wsc!(many0!(
            anychar
        )));
        let input = "a /* b */ c / * d /**/ e ";
        let output = IResult::Done("", vec!['a', 'c', '/', '*', 'd', 'e']);
        let result = test_parser(input);
        assert_eq!(output, result);
    }

    fn anychar(input: &str) -> IResult<&str, char> {
        let mut chars = input.chars();
        if let Some(c) = chars.next() {
            IResult::Done(&input[c.len_utf8()..], c)
        } else {
            IResult::Incomplete(Needed::Size(1))
        }
    }

    #[test]
    fn test_anychar() {
        let input = "Привет";
        let output = IResult::Done("ривет", 'П');
        let result = anychar(input);
        assert_eq!(output, result);
    }
}
