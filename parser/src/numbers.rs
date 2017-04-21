use nom::{IResult, Needed, ErrorKind};
use nom::hex_digit;

named!(prefixed_hex<&str, u64>, map_res!(
    preceded!(
        alt_complete!(
            tag!("0x") | tag!("0X")
        ),
        hex_digit
    ),
    |num: &str| u64::from_str_radix(num, 16)
));

fn is_num_or_suffix(c: char) -> bool {
    match c {
        '0'...'9' | 'A'...'F' | 'a'...'f' | 'h' | 'H' | 'o' | 'O' => true,
        _ => false,
    }
}

named!(suffixed_num<&str, u64>, map_res!(
    take_while1!(is_num_or_suffix),
    |num: &str| {
        match num.char_indices().last() {
            Some((0, _)) => u64::from_str_radix(num, 10),
            Some((n, 'b')) | Some((n, 'B')) => u64::from_str_radix(&num[..n], 2),
            Some((n, 'o')) | Some((n, 'O')) => u64::from_str_radix(&num[..n], 8),
            Some((n, 'd')) | Some((n, 'D')) => u64::from_str_radix(&num[..n], 10),
            Some((n, 'h')) | Some((n, 'H')) => u64::from_str_radix(&num[..n], 16),
            Some((_, _)) => {
                match num.chars().next() {
                    Some('0') => u64::from_str_radix(num, 8),
                    _ => u64::from_str_radix(num, 10),
                }
            },
            None => panic!("num is empty"),
        }
    }
));

named!(pub number<&str, u64>, do_parse!(
    num: alt_complete!(
        prefixed_hex | suffixed_num
    )
    >>
    mul: opt_complete!(alt_complete!(
        tag!("K") | tag!("M")
    ))
    >>
    (match mul {
        Some("K") => num * 1024,
        Some("M") => num * 1024 * 1024,
        None => num,
        _ => panic!("invalid multiplier")
    })
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use numbers::number;

    #[test]
    fn test_number() {
        assert_eq!(number("1234567890"), IResult::Done("", 1234567890u64));
        assert_eq!(number("0123"), IResult::Done("", 83u64));
        assert_eq!(number("0xdead"), IResult::Done("", 57005u64));

        assert_eq!(number("10K"), IResult::Done("", 0x2800u64));
        assert_eq!(number("012K"), IResult::Done("", 10_240u64));
        assert_eq!(number("0xaK"), IResult::Done("", 10_240u64));

        assert_eq!(number("10M"), IResult::Done("", 0xA0_0000u64));
        assert_eq!(number("012M"), IResult::Done("", 10_485_760u64));
        assert_eq!(number("0xAM"), IResult::Done("", 10_485_760u64));

        assert_eq!(number("0b"), IResult::Done("", 0u64));
        assert_eq!(number("0O"), IResult::Done("", 0u64));
        assert_eq!(number("0d"), IResult::Done("", 0u64));
        assert_eq!(number("0H"), IResult::Done("", 0u64));
        assert_eq!(number("0"), IResult::Done("", 0u64));
        assert_eq!(number("ah"), IResult::Done("", 10u64));
    }
}
