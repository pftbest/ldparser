use nom::multispace;

named!(comment, delimited!(
    tag!("/*"),
    take_until!("*/"),
    tag!("*/")
));

named!(pub opt_space<()>, fold_many0!(
    alt!(multispace | comment),
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
  
    fn is_not_space(c: u8) -> bool {
        c as char != ' '
    }

    #[test]
    fn test_wsc() {
        named!(test_parser<Vec<&[u8]>>, wsc!(many0!(
            take_while!(is_not_space)
        )));

        let input = b"a /* b */ c / * d /**/ e ";
        assert_done!(test_parser(input), 6);
    }

    #[test]
    fn test_opt_space() {
        named!(test_parser, do_parse!(
            tag!("(")
            >>
            opt_space
            >>
            res: take_while!(is_not_space)
            >>
            opt_space
            >>
            tag!(")")
            >>
            (res)
        ));

        let input1 = b"(  a  )";
        assert_done!(test_parser(input1));
        
        let input2 = b"(a)";
        assert_done!(test_parser(input2));
    }
}
