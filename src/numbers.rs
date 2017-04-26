use nom::{IResult, hex_digit};

fn mul_suffix(input: &str) -> IResult<&str, u64> {
    match input.chars().next() {
        Some('k') | Some('K') => IResult::Done(&input[1..], 1024),
        Some('m') | Some('M') => IResult::Done(&input[1..], 1024 * 1024),
        _ => IResult::Done(&input[..], 1),
    }
}

named!(prefixed_hex<&str, u64>, map_res!(
    do_parse!(
        alt_complete!(
            tag!("0x") | tag!("0X")
        )
        >>
        num: hex_digit
        >>
        mul: mul_suffix
        >>
        (num, mul)
    ),
    |(num, mul)| u64::from_str_radix(num, 16).map(|v| v * mul)
));

fn is_num_or_suffix(c: char) -> bool {
    match c {
        '0'...'9' | 'A'...'F' | 'a'...'f' | 'h' | 'H' | 'o' | 'O' | 'k' | 'K' | 'm' | 'M' => true,
        _ => false,
    }
}

fn parse_oct_or_dec(num: &str) -> Result<u64, ::std::num::ParseIntError> {
    match num.chars().next() {
        Some('0') => u64::from_str_radix(num, 8),
        _ => u64::from_str_radix(num, 10),
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
            Some((n, 'k')) | Some((n, 'K')) => parse_oct_or_dec(&num[..n]).map(|v| v * 1024),
            Some((n, 'm')) | Some((n, 'M')) => parse_oct_or_dec(&num[..n]).map(|v| v * 1024 * 1024),
            Some((_, _)) => parse_oct_or_dec(num),
            None => panic!("num is empty"),
        }
    }
));

named!(pub number<&str, u64>, alt_complete!(
    prefixed_hex | suffixed_num
));

#[cfg(test)]
mod test {
    use numbers::*;

    #[test]
    fn test_binary() {
        assert_done!(number("0b"), 0);
        assert_done!(number("1101b"), 0b1101);
        assert_done!(number("1101B"), 0b1101);

        assert_done!(number("1111111111111111111111111111111111111111111111111111111111111111b"),
                     0xffffffffffffffff);
        assert_fail!(number("10000000000000000000000000000000000000000000000000000000000000000b"));
        assert_done!(number("11111111111111111111111111111111b"), 0xffffffff);
        assert_done!(number("100000000000000000000000000000000b"), 0x100000000);

        assert_fail!(number("2b"));
        assert_fail!(number("ab"));

        assert_fail!(number("1101bk"));
        assert_fail!(number("1101bm"));
        assert_fail!(number("1101Bk"));
        assert_fail!(number("1101Bm"));
    }

    #[test]
    fn test_octal() {
        assert_done!(number("0o"), 0);
        assert_done!(number("123o"), 0o123);
        assert_done!(number("123O"), 0o123);

        assert_done!(number("0123k"), 0o123 * 1024);
        assert_done!(number("0123K"), 0o123 * 1024);
        assert_done!(number("0123m"), 0o123 * 1024 * 1024);
        assert_done!(number("0123M"), 0o123 * 1024 * 1024);

        assert_done!(number("1777777777777777777777o"), 0xffffffffffffffff);
        assert_fail!(number("2000000000000000000000o"));
        assert_done!(number("37777777777o"), 0xffffffff);
        assert_done!(number("40000000000o"), 0x100000000);

        assert_fail!(number("8o"));
        assert_fail!(number("ao"));

        assert_fail!(number("123ok"));
        assert_fail!(number("123om"));
        assert_fail!(number("123Ok"));
        assert_fail!(number("123Om"));
    }

    #[test]
    fn test_decimal() {
        assert_done!(number("0"), 0);
        assert_done!(number("0d"), 0);
        assert_done!(number("123"), 123);
        assert_done!(number("123d"), 123);
        assert_done!(number("123D"), 123);

        assert_done!(number("123k"), 123 * 1024);
        assert_done!(number("123K"), 123 * 1024);
        assert_done!(number("123m"), 123 * 1024 * 1024);
        assert_done!(number("123M"), 123 * 1024 * 1024);

        assert_done!(number("18446744073709551615"), 0xffffffffffffffff);
        assert_fail!(number("18446744073709551616"));
        assert_done!(number("4294967295"), 0xffffffff);
        assert_done!(number("4294967296"), 0x100000000);

        assert_done!(number("18014398509481983k"), 0xfffffffffffffc00);
        assert_done!(number("17592186044415m"), 0xfffffffffff00000);

        assert_fail!(number("ad"));
        assert_fail!(number("fd"));

        assert_fail!(number("123dk"));
        assert_fail!(number("123dm"));
    }

    #[test]
    fn test_hexadecimal() {
        assert_done!(number("0h"), 0);
        assert_done!(number("0x0"), 0);
        assert_done!(number("0xafd"), 0xafd);
        assert_done!(number("0X0"), 0x0);
        assert_done!(number("0XFD"), 0xFD);
        assert_done!(number("123h"), 0x123);
        assert_done!(number("123H"), 0x123);

        assert_done!(number("a123h"), 0xa123);
        assert_done!(number("A123H"), 0xA123);

        assert_done!(number("0xafdk"), 0xafd * 1024);
        assert_done!(number("0xafdK"), 0xafd * 1024);
        assert_done!(number("0xafdm"), 0xafd * 1024 * 1024);
        assert_done!(number("0xafdM"), 0xafd * 1024 * 1024);

        assert_done!(number("0xffffffffffffffff"), 0xffffffffffffffff);
        assert_fail!(number("0x10000000000000000"));
        assert_done!(number("0xffffffff"), 0xffffffff);
        assert_done!(number("0x100000000"), 0x100000000);

        assert_done!(number("0x3fffffffffffffk"), 0xfffffffffffffc00);
        assert_done!(number("0xfffffffffffm"), 0xfffffffffff00000);

        assert_fail!(number("123hk"));
        assert_fail!(number("123hm"));
        assert_fail!(number("123HK"));
        assert_fail!(number("123HM"));
        assert_fail!(number("0x123h"));
    }
}
