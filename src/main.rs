#[macro_use]
extern crate nom;
use std::str;

named!(comment<&str, &str>, delimited!(
        tag_s!("/*"),
        take_until_s!("*/"),
        tag_s!("*/")
));

fn spc(input: &str) -> nom::IResult<&str, &str> {
    nom::IResult::Done(&input[..10], &input[10..])
}

macro_rules! wsc(
    ($i:expr, $($args:tt)*) => ({sep!($i, spc, $($args)*)})
);

#[derive(Debug)]
enum Statement {
    Command { name: String, args: Vec<String> },
}

fn main() {
    let first_line = include_str!("/Users/vadzim/Downloads/rust/rust_on_msp/ldscripts/msp430g2553.ld"); //"a  b /* dddd */ c";

    named!(command_stmt<&str, Statement>, map!(
        nom::alphanumeric,
        |s: &str| Statement::Command{name: s.into(), args: Vec::new()}
    ));

    named!(statement<&str, Statement>, alt!(
        command_stmt
    ));

    named!(script<&str, Vec<Statement> >, wsc!(many0!(statement)));

    let res = script(first_line);
    println!("{:?}", res);
}
