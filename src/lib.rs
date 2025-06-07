//! Linker Script parser
//!
//! # Usage
//!
//! ```
//! extern crate ldscript_parser as lds;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! fn main() {
//!     let script = &mut String::new();
//!     File::open("tests/msp430bt5190.ld").unwrap()
//!                 .read_to_string(script).unwrap();
//!
//!     println!("{:#?}", lds::parse(script).unwrap());
//! }
//! ```
//!
//! # References
//!
//! - [GNU binutils documentation](https://sourceware.org/binutils/docs/ld/Scripts.html#Scripts)
//!

extern crate nom;

#[macro_use]
mod utils;
#[macro_use]
mod whitespace;
mod commands;
mod eval;
mod expressions;
mod idents;
mod memory;
mod numbers;
mod script;
mod sections;
mod statements;

pub use commands::Command;
pub use expressions::BinaryOperator;
pub use expressions::Expression;
pub use expressions::UnaryOperator;
pub use memory::Region;
pub use script::RootItem;
pub use sections::DataType;
pub use sections::OutputSectionCommand;
pub use sections::OutputSectionConstraint;
pub use sections::OutputSectionType;
pub use sections::SectionCommand;
pub use sections::SectionPattern;
pub use statements::AssignOperator;
pub use statements::Statement;

/// Parses the string that contains a linker script
pub fn parse(ldscript: &str) -> Result<Vec<RootItem>, String> {
    match script::parse(ldscript) {
        Ok((_, result)) => Ok(result),
        //TODO: add error handling
        Err(e) => Err(format!("Parsing failed, error: {:?}", e)),
    }
}
