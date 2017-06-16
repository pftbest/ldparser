use nom::{IResult, Needed, ErrorKind};

named!(pub string<&str, &str>, recognize!(delimited!(
    tag!("\""),
    take_until!("\""),
    tag!("\"")
)));

fn simple(input: &str) -> IResult<&str, &str> {
    let mut iter = input.char_indices();
    if let Some((_, c)) = iter.next() {
        // starts with a letter, underscore, or period
        if !(c.is_alphabetic() || c == '_' || c == '.') {
            return IResult::Error(ErrorKind::Char);
        }
    } else {
        return IResult::Incomplete(Needed::Size(1));
    }
    for (i, c) in iter {
        // may include letters, digits, underscores, periods, and hyphens
        if !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-') {
            return IResult::Done(&input[i..], &input[..i]);
        }
    }
    IResult::Done(&input[input.len()..], &input[..])
}

named!(pub symbol<&str, &str>, alt!(
     string | simple
));

fn is_pattern(c: char) -> bool {
    c.is_alphanumeric() || "_.$/\\~=+[]*?-!<>^:".contains(c)
}

named!(simple_pattern<&str, &str>, take_while1!(
    is_pattern
));

named!(pub pattern<&str, &str>, alt!(
    string | simple_pattern
));

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
            "\"spaces are ok, just quote the identifier\""
        );
    }

    #[test]
    fn test_pattern() {
        assert_done!(pattern("0"), "0");
        assert_done!(pattern(".text"), ".text");
        assert_done!(pattern("hello*.o"), "hello*.o");
        assert_done!(
            pattern("\"spaces are ok, just quote the identifier\""),
            "\"spaces are ok, just quote the identifier\""
        );
        assert_done!(
            pattern("this+is-another*crazy[example]"),
            "this+is-another*crazy[example]"
        );
    }
}
