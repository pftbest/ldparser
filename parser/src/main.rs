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
mod commands;
mod statements;
mod memory;
mod sections;
mod script;

#[bench]
fn bench_some(b: &mut ::test::Bencher) {
    b.iter(|| {
               let x = ::test::black_box("x = ((a((1) - - - (2) - - - (3)) + 1));");
               script::parse(x)
           })
}

fn main() {
    let x = r"
    MEMORY
       {
         rom (rx)  : ORIGIN = 0, LENGTH = 256K
         ram (!rx) : org = 0x40000000, l = 4M
       }
    ";
    println!("{:#?}", script::parse(x));
}
