use std::ops::{Range, RangeFrom, RangeTo};
use nom::{IResult, Needed, Slice, InputIter, InputLength, ErrorKind, AsChar};

pub const SYMBOL_NAME_ERROR: u32 = 1;
pub const FILE_NAME_ERROR: u32 = 2;

named!(pub symbol_name<&str, &str>, alt_complete!(
    identifier | delimited!(
        tag_s!("\""),
        take_until_s!("\""),
        tag_s!("\"")
    )
));

/// Recognizes symbol name. Starts with a letter, underscore, or point and may
/// include any letters, underscores, digits, points, and hyphens.
fn identifier<T>(input: T) -> IResult<T, T>
    where T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
          T: InputIter + InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }
    for (idx, item) in input.iter_indices() {
        let c = item.as_char();
        if idx == 0 {
            if !(c.is_alphabetic() || c == '_' || c == '.') {
                return IResult::Error(error_position!(ErrorKind::Custom(SYMBOL_NAME_ERROR), input));
            }
        } else {
            if !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-') {
                return IResult::Done(input.slice(idx..), input.slice(0..idx));
            }
        }
    }
    IResult::Done(input.slice(input_length..), input)
}

/// Recognizes file name pattern. May contain '*', '?' and ':'.
pub fn file_name<T>(input: T) -> IResult<T, T>
    where T: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
          T: InputIter + InputLength
{
    let input_length = input.input_len();
    if input_length == 0 {
        return IResult::Incomplete(Needed::Unknown);
    }
    for (idx, item) in input.iter_indices() {
        let c = item.as_char();
        if !(c.is_alphanumeric() || c == '_' || c == '.' || c == '-' || c == '*' || c == ':' ||
             c == '?') {
            if idx == 0 {
                return IResult::Error(error_position!(ErrorKind::Custom(FILE_NAME_ERROR), input));
            } else {
                return IResult::Done(input.slice(idx..), input.slice(0..idx));
            }
        }
    }
    IResult::Done(input.slice(input_length..), input)
}

#[cfg(test)]
mod test {
    use nom::{IResult, ErrorKind};
    use symbols::{symbol_name, file_name};

    #[test]
    fn test_symbol_name() {
        assert_eq!(symbol_name("."), IResult::Done("", "."));
        assert_eq!(symbol_name("A6*"), IResult::Done("*", "A6"));
        assert_eq!(symbol_name(".a_b-c.0 "), IResult::Done(" ", ".a_b-c.0"));
        assert_eq!(symbol_name("-"), IResult::Error(ErrorKind::Alt));
        assert_eq!(symbol_name("5"), IResult::Error(ErrorKind::Alt));
        assert_eq!(symbol_name("\"5\""), IResult::Done("", "5"));
        assert_eq!(file_name("*crtbegin*.o "),
                   IResult::Done(" ", "*crtbegin*.o"));
    }
}
