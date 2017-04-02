#[macro_use]
extern crate nom;

#[macro_use]
mod whitespace;

mod numbers;
mod symbols;
mod expressions;
mod statements;
mod memory;
mod sections;
mod commands;

pub fn parse(script: &str) {
    commands::script(script).unwrap();
}
