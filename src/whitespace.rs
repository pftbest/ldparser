use nom::multispace;

named!(comment<&str, &str>, delimited!(
    tag!("/*"),
    take_until!("*/"),
    tag!("*/")
));

named!(space_or_comment<&str, &str>, alt!(
    multispace | comment
));

named!(pub space<&str, ()>, fold_many1!(
    space_or_comment,
    (),
    |_, _| ()
));

named!(pub opt_space<&str, ()>, fold_many0!(
    space_or_comment,
    (),
    |_, _| ()
));

/// Transforms a parser to automatically consume whitespace and comments
/// between each token.
macro_rules! wsc(
    ($i:expr, $($args:tt)*) => ({
        use $crate::whitespace::opt_space;
        sep!($i, opt_space, $($args)*)
    })
);

#[cfg(test)]
mod tests {
    use whitespace::opt_space;

    fn is_good(c: char) -> bool {
        c.is_alphanumeric() || c == '/' || c == '*'
    }

    #[test]
    fn test_wsc() {
        named!(test_parser<&str, Vec<&str>>, wsc!(many0!(
            take_while!(is_good)
        )));

        let input = "a /* b */ c / * d /**/ e ";
        assert_done!(test_parser(input), vec!["a", "c", "/", "*", "d", "e"]);
    }

    #[test]
    fn test_opt_space() {
        named!(test_parser<&str, &str>, do_parse!(
            tag!("(")
            >>
            opt_space
            >>
            res: take_while!(is_good)
            >>
            opt_space
            >>
            tag!(")")
            >>
            (res)
        ));

        let input1 = "(  a  )";
        assert_done!(test_parser(input1));

        let input2 = "(a)";
        assert_done!(test_parser(input2));
    }
}
