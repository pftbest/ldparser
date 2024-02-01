#[allow(unused_macros)]
macro_rules! assert_done {
    ($res:expr) => {{
        use nom::Finish;
        match $res.finish() {
            Ok(_) => {}
            Err(r) => panic!("fail: {:?}", r),
        }
    }};
    ($res:expr, $expected:expr) => {{
        use nom::Finish;
        match $res.finish() {
            Ok((_, out)) => {
                assert_eq!(out, $expected);
            }
            Err(r) => panic!("fail: {:?}", r),
        }
    }};
}

#[allow(unused_macros)]
macro_rules! assert_done_vec {
    ($res:expr, $num:expr) => {{
        use nom::Finish;
        match $res.finish() {
            Ok((_, v)) => {
                assert_eq!(v.len(), $num);
            }
            Err(r) => panic!("fail: {:?}", r),
        }
    }};
}

#[allow(unused_macros)]
macro_rules! assert_fail {
    ($res:expr) => {{
        use nom::Finish;
        match $res.finish() {
            Ok((_, out)) => {
                panic!("should fail: {:?} got {:?}", $res, out)
            }
            _ => {}
        }
    }};
}
