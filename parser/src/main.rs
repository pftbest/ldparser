#![feature(test)]
extern crate test;

#[macro_use]
extern crate nom;

#[macro_use]
mod utils;
#[macro_use]
mod whitespace;
mod numbers;
mod idents;
mod expressions;
mod statements;

#[bench]
fn bench_some(b: &mut ::test::Bencher) {
    b.iter(|| {
               let x = ::test::black_box("x = ((a((1) - - - (2) - - - (3)) + 1));");
               statements::statement(x)
           })
}

fn main() {
    let x = "((a((1) - - - (2) - - - (3)) + 1))";
    println!("{:#?}", statements::statement(x));
}
