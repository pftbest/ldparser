use nom::digit;
use nom::alphanumeric;
use std::str::FromStr;

named!(pub symbol<&str, &str>, call!(alphanumeric));

named!(pub number<&str, u64>,
    map_res!(
        digit,
        FromStr::from_str
    )
);

macro_rules! assert_done {
    ($res:expr) => (
        match $res {
            ::nom::IResult::Done("", _) => {},
            r @ _ => panic!("fail: {:?}", r),
        }
    );
    ($res:expr, $num:expr) => (
        match $res {
            ::nom::IResult::Done("", v @ _) => {
                assert_eq!(v.len(), $num);
            },
            r @ _ => panic!("fail: {:?}", r),
        }
    );
}
