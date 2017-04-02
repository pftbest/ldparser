use symbols::{symbol_name, file_name};
use statements::{statement, Statement};

#[derive(Debug, PartialEq)]
pub enum InputSection {
    Section(String),
    ExcludeFile(String),
}

#[derive(Debug, PartialEq)]
pub enum SectionDef {
    Statement(Statement),
    Wildcard(Vec<InputSection>),
    FileSections(Vec<InputSection>),
    FileNames(Vec<String>),
    Keep(Box<SectionDef>)
}

named!(section_name<&str, InputSection>, map!(
    file_name,
    |x| InputSection::Section(x.into())
));

named!(sect_wildcard<&str, SectionDef>, map!(
    statement,
    |x| SectionDef::Statement(x)
));

named!(sect_statement<&str, SectionDef>, map!(
    statement,
    |x| SectionDef::Statement(x)
));

named!(section<&str, SectionDef>, alt_complete!(
    sect_statement
));

named!(pub sections<&str, Vec<SectionDef>>, many0!(
    section
));
