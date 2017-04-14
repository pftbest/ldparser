use std::str::FromStr;
use nom::{IResult, Needed, ErrorKind};
use nom::{digit, hex_digit, oct_digit};

named!(decimal<&str, u64>, map_res!(
    digit,
    FromStr::from_str
));

named!(hexadecimal<&str, u64>, preceded!(
    tag_no_case!("0x"),
    map_res!(
        hex_digit,
        |x| u64::from_str_radix(x, 16)
    )
));

named!(octal<&str, u64>, preceded!(
    tag!("0"),
    map_res!(
        oct_digit,
        |x| u64::from_str_radix(x, 8)
    )
));

named!(simple<&str, u64>,
    alt_complete!(hexadecimal | octal | decimal)
);

named!(kilo<&str, u64>, map!(
    terminated!(
        simple,
        tag!("K")
    ),
    |x| (x * 1_024)
));

named!(mega<&str, u64>, map!(
    terminated!(
        simple,
        tag!("M")
    ),
    |x| (x * 1_048_576)
));

named!(pub number<&str, u64>,
    alt_complete!(kilo | mega | simple)
);

#[derive(Debug, PartialEq)]
enum Attribute {
    Hex,
    Oct,
    Bin,
    Dec,
    Kilo,
    Mega,
}

fn suffix(input: &str) -> IResult<&str, Attribute> {
    let mut iter = input.chars();
    let first = iter.next();
    match first {
        Some(c) => {
            match c {
                'h' | 'H' => IResult::Done(iter.as_str(), Attribute::Hex),
                'o' | 'O' => IResult::Done(iter.as_str(), Attribute::Oct),
                'b' | 'B' => IResult::Done(iter.as_str(), Attribute::Bin),
                'd' | 'D' => IResult::Done(iter.as_str(), Attribute::Dec),
                'k' | 'K' => IResult::Done(iter.as_str(), Attribute::Kilo),
                'm' | 'M' => IResult::Done(iter.as_str(), Attribute::Mega),
                _ => IResult::Error(ErrorKind::Char),
            }
        },
        None => IResult::Incomplete(Needed::Size(1)),
    }
}

named!(prefix<&str, Attribute>, map!(
    alt_complete!(
        tag!("0x") | tag!("0X") | tag!("0")
    ),
    |x| match x {
        "0x" | "0X" => Attribute::Hex,
        "0" => Attribute::Oct,
        _ => panic!("wrong prefix"),
    }
));

named!(pub num<&str, u64>, do_parse!(
    prefix: opt_complete!(prefix)
    >>
    body: hex_digit
    >>
    suffix: opt_complete!(suffix)
    >>
    (
        match (prefix, suffix, body) {
            (Oct, None, "") => 0,
            (_, _, _) => 1,
        }
    )
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
    }
}
