use statements::{assignment, Statement};
use commands::{command, Command};
use idents::symbol;
use idents::pattern;
use expressions::Expression;
use expressions::expression;
use whitespace::opt_space;

#[derive(Debug, PartialEq)]
pub enum SectionCommand {
    Statement(Statement),
    Command(Command),
    OutputSection {
        name: String,
        vma_address: Option<Box<Expression>>,
        s_type: Option<OutputSectionType>,
        lma_address: Option<Box<Expression>>,
        section_align: Option<Box<Expression>>,
        align_with_input: bool,
        subsection_align: Option<Box<Expression>>,
        constraint: Option<OutputSectionConstraint>,
        list: Vec<OutputSectionCommand>,
        region: Option<String>,
        lma_region: Option<String>,
        fillexp: Option<Box<Expression>>,
    },
}

#[derive(Debug, PartialEq)]
pub enum OutputSectionCommand {
    Statement(Statement),
    Data {
        d_type: DataType,
        value: Box<Expression>,
    },
    InputSection {
        file: SectionPattern,
        sections: Vec<SectionPattern>,
    },
    KeepInputSection {
        file: SectionPattern,
        sections: Vec<SectionPattern>,
    },
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    Byte,
    Short,
    Long,
    Quad,
}

#[derive(Debug, PartialEq)]
pub enum SectionPattern {
    Simple(String),
    SortByName(String),
    SortByAlignment(String),
    SortByInitPriority(String),
    SortNone(String),
    ExcludeFile {
        files: Vec<String>,
        pattern: Box<SectionPattern>,
    },
}

#[derive(Debug, PartialEq)]
pub enum OutputSectionType {
    NoLoad,
    DSect,
    Copy,
    Info,
    Overlay,
}

#[derive(Debug, PartialEq)]
pub enum OutputSectionConstraint {
    OnlyIfRo,
    OnlyIfRw,
}

named!(output_section_type<&str, OutputSectionType>, alt_complete!(
    map!(tag!("(NOLOAD)"), |_| OutputSectionType::NoLoad) |
    map!(tag!("(DSECT)"), |_| OutputSectionType::DSect) |
    map!(tag!("(COPY)"), |_| OutputSectionType::Copy) |
    map!(tag!("(INFO)"), |_| OutputSectionType::Info) |
    map!(tag!("(OVERLAY)"), |_| OutputSectionType::Overlay)
));

named!(output_section_constraint<&str, OutputSectionConstraint>, alt_complete!(
    map!(tag!("ONLY_IF_RO"), |_| OutputSectionConstraint::OnlyIfRo) |
    map!(tag!("ONLY_IF_RW"), |_| OutputSectionConstraint::OnlyIfRw)
));

named!(sorted_sp<&str, SectionPattern>, do_parse!(
    keyword: alt_complete!(
        tag!("SORT_BY_NAME") |
        tag!("SORT_BY_ALIGNMENT") |
        tag!("SORT_BY_INIT_PRIORITY") |
        tag!("SORT_NONE") |
        tag!("SORT")
    )
    >>
    wsc!(tag!("("))
    >>
    inner: pattern
    >>
    opt_space
    >>
    tag!(")")
    >>
    (match keyword {
        "SORT" | "SORT_BY_NAME" => SectionPattern::SortByName(inner.into()),
        "SORT_BY_ALIGNMENT" => SectionPattern::SortByAlignment(inner.into()),
        "SORT_BY_INIT_PRIORITY" => SectionPattern::SortByInitPriority(inner.into()),
        "SORT_NONE" => SectionPattern::SortNone(inner.into()),
        _ => panic!("wrong sort keyword"),
    })
));

named!(exclude_file_sp<&str, SectionPattern>, do_parse!(
    tag!("EXCLUDE_FILE")
    >>
    opt_space
    >>
    tag!("(")
    >>
    files: wsc!(many1!(
        map!(pattern, String::from)
    ))
    >>
    tag!(")")
    >>
    opt_space
    >>
    inner: section_pattern
    >>
    (SectionPattern::ExcludeFile {
        files: files,
        pattern: Box::new(inner),
    })
));

named!(simple_sp<&str, SectionPattern>, map!(
    pattern,
    |x: &str| SectionPattern::Simple(x.into())
));

named!(section_pattern<&str, SectionPattern>, alt!(
    exclude_file_sp | sorted_sp | simple_sp
));

named!(data_osc<&str, OutputSectionCommand>, do_parse!(
    d_type: alt_complete!(
        tag!("BYTE") | tag!("SHORT") | tag!("LONG") | tag!("QUAD")
    )
    >>
    wsc!(tag!("("))
    >>
    value: expression
    >>
    wsc!(tag!(")"))
    >>
    opt_complete!(tag!(";"))
    >>
    (OutputSectionCommand::Data {
        d_type: match d_type {
            "BYTE" => DataType::Byte,
            "SHORT" => DataType::Short,
            "LONG" => DataType::Long,
            "QUAD" => DataType::Quad,
            _ => panic!("invalid data type")
        },
        value: Box::new(value)
    })
));

named!(assignment_osc<&str, OutputSectionCommand>, map!(
    assignment,
    |stmt| OutputSectionCommand::Statement(stmt)
));

named!(input_osc<&str, OutputSectionCommand>, do_parse!(
    file: section_pattern
    >>
    opt_space
    >>
    sections: opt_complete!(
        delimited!(
            wsc!(tag!("(")),
            wsc!(many1!(
                section_pattern
            )),
            wsc!(tag!(")"))
        )
    )
    >>
    (OutputSectionCommand::InputSection {
        file: file,
        sections: match sections {
            Some(s) => s,
            None => Vec::new(),
        },
    })
));

named!(keep_osc<&str, OutputSectionCommand>, do_parse!(
    tag!("KEEP")
    >>
    wsc!(tag!("("))
    >>
    inner: input_osc
    >>
    wsc!(tag!(")"))
    >>
    (match inner {
        OutputSectionCommand::InputSection {
            file,
            sections
        } => OutputSectionCommand::KeepInputSection {
            file: file,
            sections: sections,
        },
        _ => panic!("wrong output section command"),
    })
));

named!(output_section_command<&str, OutputSectionCommand>, alt_complete!(
    assignment_osc | keep_osc | data_osc | input_osc
));

named!(statement_sc<&str, SectionCommand>, map!(
    assignment,
    |stmt| SectionCommand::Statement(stmt)
));

named!(command_sc<&str, SectionCommand>, map!(
    command,
    |cmd| SectionCommand::Command(cmd)
));

named!(output_sc<&str, SectionCommand>, do_parse!(
    name: alt!(tag!("/DISCARD/") | symbol)
    >>
    opt_space
    >>
    s_type1: opt!(output_section_type) // ugly hack
    >>
    vma: wsc!(opt!(expression))
    >>
    s_type2: opt!(output_section_type)
    >>
    wsc!(tag!(":"))
    >>
    lma: opt!(delimited!(
        tag!("AT("),
        wsc!(expression),
        tag!(")")
    ))
    >>
    opt_space
    >>
    section_align: opt!(delimited!(
        tag!("ALIGN("),
        wsc!(expression),
        tag!(")")
    ))
    >>
    align_with_input: wsc!(opt!(
        tag!("ALIGN_WITH_INPUT")
    ))
    >>
    subsection_align: opt!(delimited!(
        tag!("SUBALIGN("),
        wsc!(expression),
        tag!(")")
    ))
    >>
    constraint: wsc!(opt!(output_section_constraint))
    >>
    wsc!(tag!("{"))
    >>
    list: wsc!(many0!(
        output_section_command
    ))
    >>
    wsc!(tag!("}"))
    >>
    region: opt_complete!(preceded!(
        tag!(">"),
        wsc!(symbol)
    ))
    >>
    lma_region: opt_complete!(preceded!(
        tag!("AT>"),
        wsc!(symbol)
    ))
    >>
    fillexp: opt_complete!(preceded!(
        tag!("="),
        wsc!(expression)
    ))
    >>
    opt_complete!(tag!(","))
    >>
    (SectionCommand::OutputSection {
        name: name.into(),
        vma_address: vma.map(Box::new),
        s_type: if s_type1.is_some() { s_type1 } else { s_type2 },
        lma_address: lma.map(Box::new),
        section_align: section_align.map(Box::new),
        align_with_input: align_with_input.is_some(),
        subsection_align: subsection_align.map(Box::new),
        constraint: constraint,
        list: list,
        region: region.map(String::from),
        lma_region: lma_region.map(String::from),
        fillexp: fillexp.map(Box::new),
    })
));

named!(pub section_command<&str, SectionCommand>, alt_complete!(
    statement_sc | output_sc | command_sc
));

#[cfg(test)]
mod tests {
    use sections::*;

    #[test]
    fn test_section_command() {
        assert_fail!(section_pattern("EXCLUDE_FILE (*a)"));
        assert_fail!(input_osc("EXCLUDE_FILE (*a)"));
        assert_done!(section_pattern("EXCLUDE_FILE ( *a *b ) .c"));
        assert_done!(input_osc("EXCLUDE_FILE ( *a *b ) *c"));

        assert_fail!(input_osc("EXCLUDE_FILE ( EXCLUDE_FILE ( *a *b ) *c ) .d"));
        assert_done!(input_osc("EXCLUDE_FILE ( *a ) *b ( .c )"));
        assert_done!(input_osc("EXCLUDE_FILE ( *a ) *b ( .c .d )"));
        assert_done!(input_osc("EXCLUDE_FILE ( *a ) *b ( .c EXCLUDE_FILE ( *a ) .d )"));

        assert_done!(output_sc("/DISCARD/ : { *(.note.GNU-stack) }"));
        assert_done!(output_sc(".DATA : { [A-Z]*(.data) }"));
        assert_done!(output_sc(".infoD     : {} > INFOD"));

        assert_done!(output_section_command("[A-Z]*(.data)"));
        assert_done!(output_section_command("LONG((__CTOR_END__ - __CTOR_LIST__) / 4 - 2)"));
        assert_done!(output_section_command("EXCLUDE_FILE (*crtend.o *otherfile.o) *(.ctors)"));
        assert_done!(output_section_command("*(EXCLUDE_FILE (*crtend.o *otherfile.o) .ctors)"));
        assert_done!(output_section_command("*(EXCLUDE_FILE (*somefile.o) .text EXCLUDE_FILE (*somefile.o) .rdata)"));
        assert_done!(output_section_command("KEEP(SORT_BY_NAME(*)(.ctors))"));
        assert_done!(output_section_command("PROVIDE (__init_array_end = .);"));
        assert_done!(output_section_command("LONG(0);"));
        assert_done!(output_section_command("SORT(CONSTRUCTORS)"));
        assert_done!(output_section_command("*"));
    }
}
