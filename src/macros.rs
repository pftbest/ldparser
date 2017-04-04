/// Transforms a parser to automatically consume whitespace and comments
/// between each token.
macro_rules! wsc(
    ($i:expr, $($args:tt)*) => ({
        use $crate::whitespace::space_or_comment;
        sep!($i, space_or_comment, $($args)*)
    })
);

macro_rules! opt_complete(
    ($i:expr, $submac:ident!( $($args:tt)* )) => ({
        use nom::IResult;
        match $submac!($i, $($args)*) {
            IResult::Done(i,o)     => IResult::Done(i, ::std::option::Option::Some(o)),
            IResult::Error(_)      => IResult::Done($i, ::std::option::Option::None),
            IResult::Incomplete(_) => IResult::Done($i, ::std::option::Option::None)
        }
    });
    ($i:expr, $f:expr) => (
        opt_complete!($i, call!($f));
    );
);
