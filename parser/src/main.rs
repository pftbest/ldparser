#![feature(test)]
extern crate test;

#[macro_use]
extern crate nom;

#[macro_use]
mod utils;
#[macro_use]
mod whitespace;
mod expressions;

use expressions::expression;

#[bench]
fn bench_some(b: &mut test::Bencher) {
    b.iter(|| {
               let x = test::black_box(b"((a((1) - - - (2) - - - (3)) + 1))");
               expression(x)
           })
}

fn main() {
    let x = b"((a((1) - - - (2) - - - (3)) + 1))";
    println!("{:#?}", expression(x));
}
