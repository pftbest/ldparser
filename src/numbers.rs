use std::panic;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{hex_digit1, one_of},
    combinator::{map, map_res, opt},
    error::{ErrorKind, ParseError},
    Err, IResult,
};

fn mul_suffix0(input: &str) -> IResult<&str, u64> {
    map(opt(one_of("kKmM")), |chr: Option<char>| {
        chr.map_or(1, |c: char| match c {
            'k' | 'K' => 1024,
            'm' | 'M' => 1024 * 1024,
            _ => panic!("Should not happen"),
        })
    })(input)
}

fn prefixed_hex(input: &str) -> IResult<&str, u64> {
    let (_input, _) = (alt((tag("0x"), tag("0X"))))(input)?;
    let (_input, num) = hex_digit1(_input)?;
    match u64::from_str_radix(num, 16) {
        Ok(val) => {
            let (_input, mul) = mul_suffix0(_input)?;
            if let Some(chr) = _input.chars().next() {
                if chr.is_alphanumeric() {
                    return Err(Err::Failure(ParseError::from_error_kind(
                        input,
                        ErrorKind::HexDigit,
                    )));
                }
            }
            match val.checked_mul(mul) {
                Some(v) => Ok((_input, v)),
                None => Err(Err::Failure(ParseError::from_error_kind(
                    input,
                    ErrorKind::HexDigit,
                ))),
            }
        }
        Err(_) => Err(Err::Failure(ParseError::from_error_kind(
            input,
            ErrorKind::HexDigit,
        ))),
    }
}

fn is_num_or_suffix(c: char) -> bool {
    match c {
        '0'..='9' | 'A'..='F' | 'a'..='f' | 'h' | 'H' | 'o' | 'O' | 'k' | 'K' | 'm' | 'M' => true,
        _ => false,
    }
}

fn parse_oct_or_dec(num: &str) -> Result<u64, ::std::num::ParseIntError> {
    match num.chars().next() {
        Some('0') => u64::from_str_radix(num, 8),
        _ => u64::from_str_radix(num, 10),
    }
}

fn suffixed_num(input: &str) -> IResult<&str, u64> {
    map_res(take_while1(is_num_or_suffix), |num: &str| {
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
    })(input)
}

pub fn number(input: &str) -> IResult<&str, u64> {
    alt((prefixed_hex, suffixed_num))(input)
}

#[cfg(test)]
mod test {
    use numbers::*;

    #[test]
    fn test_binary() {
        assert_done!(number("0b"), 0);
        assert_done!(number("1101b"), 0b1101);
        assert_done!(number("1101B"), 0b1101);

        assert_done!(
            number("1111111111111111111111111111111111111111111111111111111111111111b",),
            0xffffffffffffffff
        );
        assert_fail!(number(
            "10000000000000000000000000000000000000000000000000000000000000000b",
        ));
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
