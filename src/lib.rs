//! Linker Script parser
//!
//! # Usage
//!
//! ``` no_run
//! extern crate ldscript_parser as lds;
//!
//! use std::fs::File;
//! use std::io::Read;
//!
//! fn main() {
//!     let script = &mut String::new();
//!     File::open("cortex-m4.ld").unwrap().read_to_string(script);
//!
//!     println!("{:#?}", lds::parse(script));
//! }
//! ```
//!
//! # References
//!
//! - [GNU binutils documentation](https://sourceware.org/binutils/docs/ld/Scripts.html#Scripts)
//!

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

pub use expressions::UnaryOperator;
pub use expressions::BinaryOperator;
pub use expressions::Expression;
pub use commands::Command;
pub use statements::AssignOperator;
pub use statements::Statement;
pub use memory::Region;
pub use sections::DataType;
pub use sections::SectionPattern;
pub use sections::OutputSectionType;
pub use sections::OutputSectionConstraint;
pub use sections::OutputSectionCommand;
pub use sections::SectionCommand;
pub use script::RootItem;

/// Parses the string that contains a linker script
pub fn parse(ldscript: &str) -> Result<Vec<RootItem>, String> {
    match script::parse(ldscript) {
        nom::IResult::Done("", result) => Ok(result),
        //TODO: add error handling
        err => Err(format!("Parsing failed, error: {:?}", err)),
    }
}
