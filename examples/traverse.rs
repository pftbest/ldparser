extern crate ldparser;

use std::io;
use std::io::Read;
use std::fs::File;
use ldparser::{Command, Section, OutputSection};

fn main() {
    let script = load_script("tests/cc430f5137.ld").unwrap();
    let items = ldparser::parse(&script).unwrap();
    traverse(items);
}

fn load_script(name: &str) -> io::Result<String> {
    let mut file = try!(File::open(name));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(contents)
}

fn traverse(root_items: Vec<Command>) {
    for cmd in root_items {
        match cmd {
            Command::Memory(regions) => {
                for region in regions {
                    println!("Memory region: {:?}", region);
                }
            },
            Command::Sections(sections) => {
                for def in sections {
                    match def {
                        Section::Definition(out_section) => {
                            traverse_out_section(out_section);
                        },
                        Section::Statement(statement) => {
                            println!("Statement in SECTIONS: {:?}", statement);
                        }
                    }
                }
            },
            c @ _ => {
                // other commands
                println!("Command: {:?}", c);
            }
        }
    }
}

fn traverse_out_section(outs: OutputSection) {
    println!("Output section: {}", outs.name);
    if let Some(a) = outs.load_address {
        println!("\tload_address: {:?}", a);
    }
    if let Some(a) = outs.align {
        println!("\talign: {:?}", a);
    }
    if let Some(a) = outs.attribute {
        println!("\tattribute: {}", a);
    }
    if outs.no_load {
        println!("\tno_load: true");
    }
    if let Some(a) = outs.start {
        println!("\tstart: {:?}", a);
    }
    if let Some(a) = outs.region {
        println!("\tregion: {}", a);
    }
    if let Some(a) = outs.region_at {
        println!("\tregion_at: {}", a);
    }
    if let Some(a) = outs.fill {
        println!("\tfill: {:?}", a);
    }
    for item in outs.contents {
        println!("\tItem: {:?}", item);
    }
}
