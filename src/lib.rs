#[macro_use]
extern crate nom;

#[macro_use]
mod macros;

mod whitespace;
mod numbers;
mod symbols;
mod expressions;
mod statements;
mod memory;
mod sections;
mod commands;

#[cfg(test)]
mod tests;

pub use commands::Command;
pub use sections::{Section, InputSection, OutputSection, SectionItem};
pub use memory::Region;
pub use statements::Statement;
pub use expressions::Expression;

use nom::IResult;

pub fn parse(text: &str) -> Result<Vec<Command>, String> {
    match commands::script(text) {
        IResult::Done("", v) => Ok(v),
        r @ _ => Err(format!("Parsing failed: {:?}", r)),
    }
}
