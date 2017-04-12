use nom::digit;
use nom::alphanumeric;
use std::str::FromStr;
use std::str;

named!(pub symbol, call!(alphanumeric));

named!(pub number<u64>,
    map_res!(
        map_res!(
            digit,
            str::from_utf8
        ),
        FromStr::from_str
    )
);

macro_rules! assert_done {
    ($res:expr) => (
        match $res {
            ::nom::IResult::Done(b"", _) => {},
            r @ _ => panic!("fail: {:?}", r),
        }
    );
    ($res:expr, $num:expr) => (
        match $res {
            ::nom::IResult::Done(b"", v @ _) => {
                assert_eq!(v.len(), $num);
            },
            r @ _ => panic!("fail: {:?}", r),
        }
    );
}
