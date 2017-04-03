use symbols::{file_name, symbol_name};
use expressions::{expression, value, Expression};
use statements::{statement, Statement};

#[derive(Debug, PartialEq)]
pub enum InputSection {
    Section(String),
    Command {
        name: String,
        args: Vec<String>
    }
}

#[derive(Debug, PartialEq)]
pub enum SectionItem {
    Statement(Statement),
    Sections {
        file: String,
        sections: Vec<InputSection>,
    },
    File(String),
    Keep(Box<SectionItem>),
}

#[derive(Debug, PartialEq)]
pub struct OutputSection {
    pub name: String,
    pub start: Option<Expression>,
    pub align: Option<Expression>,
    pub no_load: bool,
    pub load_address: Option<Expression>,
    pub contents: Vec<SectionItem>,
    pub region: Option<String>,
    pub region_at: Option<String>,
    pub fill: Option<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum Section {
    Definition(OutputSection),
    Statement(Statement),
}

named!(input_section_name<&str, InputSection>, map!(
    file_name,
    |x: &str| InputSection::Section(x.into())
));

named!(input_section_command<&str, InputSection>, wsc!(do_parse!(
    name: symbol_name
    >>
    tag_s!("(")
    >>
    args: wsc!(many1!(file_name))
    >>
    tag_s!(")")
    >>
    (InputSection::Command{
        name: name.into(),
        args: args.iter().map(|x| String::from(*x)).collect::<Vec<_>>()
    })
)));

named!(input_section<&str, InputSection>, alt_complete!(
    input_section_command | input_section_name
));

named!(section_item_sections<&str, SectionItem>, wsc!(do_parse!(
    name: file_name
    >>
    tag_s!("(")
    >>
    sections: wsc!(many1!(input_section))
    >>
    tag_s!(")")
    // >>
    // opt!(complete!(tag_s!(";")))
    >>
    (SectionItem::Sections{
        file: name.into(),
        sections: sections,
    })
)));

named!(section_item_file<&str, SectionItem>, map!(
    file_name,
    |x: &str| SectionItem::File(x.into())
));

named!(section_item_keep<&str, SectionItem>, wsc!(do_parse!(
    tag_s!("KEEP")
    >>
    tag_s!("(")
    >>
    item: section_item
    >>
    tag_s!(")")
    // >>
    // opt!(tag_s!(";"))
    >>
    (SectionItem::Keep(Box::new(item)))
)));

named!(section_item_statement<&str, SectionItem>, map!(
    statement,
    |x| SectionItem::Statement(x)
));

named!(section_item<&str, SectionItem>, alt_complete!(
    section_item_keep | section_item_sections | section_item_statement | section_item_file
));

named!(section_items<&str, Vec<SectionItem>>, many0!(
    section_item
));

named!(align<&str, Expression>, wsc!(do_parse!(
    tag_s!("BLOCK")
    >>
    tag_s!("(")
    >>
    expr: expression
    >>
    tag_s!(")")
    >>
    (expr)
)));

named!(load_addr<&str, Expression>, wsc!(do_parse!(
    tag_s!("AT")
    >>
    tag_s!("(")
    >>
    expr: expression
    >>
    tag_s!(")")
    >>
    (expr)
)));

named!(region<&str, String>, wsc!(do_parse!(
    tag_s!(">")
    >>
    name: symbol_name
    >>
    (name.into())
)));

named!(region_at<&str, String>, wsc!(do_parse!(
    tag_s!("AT")
    >>
    tag_s!(">")
    >>
    name: symbol_name
    >>
    (name.into())
)));

named!(fill<&str, Expression>, wsc!(do_parse!(
    tag_s!("=")
    >>
    expr: value
    >>
    (expr)
)));

named!(output_section<&str, OutputSection>, wsc!(do_parse!(
    sect_name: alt_complete!(tag_s!("/DISCARD/") | symbol_name)
    >>
    start: opt!(value)
    >>
    align: opt!(align)
    >>
    no_load: opt!(tag_s!("(NOLOAD)"))
    >>
    tag_s!(":")
    >>
    load_address: opt!(load_addr)
    >>
    tag_s!("{")
    >>
    items: section_items
    >>
    tag_s!("}")
    >>
    region: opt!(region)
    >>
    region_at: opt!(region_at)
    >>
    fill: opt!(fill)
    >>
    (OutputSection{
        name: sect_name.into(),
        start: start,
        align: align,
        no_load: no_load.is_some(),
        load_address: load_address,
        contents: items,
        region: region,
        region_at: region_at,
        fill: fill,
    })
)));

named!(sect_statement<&str, Section>, map!(
    statement,
    |x| Section::Statement(x)
));

named!(sect_definition<&str, Section>, map!(
    output_section,
    |x| Section::Definition(x)
));

named!(pub sections<&str, Vec<Section>>, many0!(alt_complete!(
    sect_definition | sect_statement
)));

#[cfg(test)]
mod test {
    use nom::IResult;
    use statements::Statement;
    use expressions::Expression::Number;
    use sections::{section_items, sections, SectionItem, InputSection};

    #[test]
    fn test_sections() {
        match section_items(" a ( b* .g )  KEEP ( * ( SORT ( c ) ) ) foo.o . = 0 ; ") {
            IResult::Done("", v) => {
                assert_eq!(v.len(), 4);

                assert_eq!(v[0],
                           SectionItem::Sections {
                               file: String::from("a"),
                               sections: vec![InputSection::Section(String::from("b*")),
                                              InputSection::Section(String::from(".g"))],
                           });

                assert_eq!(v[1], SectionItem::Keep(Box::new(
                    SectionItem::Sections{
                    file: String::from("*"),
                    sections: vec![InputSection::Command{
                        name: String::from("SORT"),
                        args: vec![String::from("c")]
                    }]
                })));

                assert_eq!(v[2], SectionItem::File(String::from("foo.o")));

                assert_eq!(v[3],
                           SectionItem::Statement(Statement::Assign {
                                                      symbol: String::from("."),
                                                      operator: String::from("="),
                                                      expr: Number(0),
                                                  }));
            }
            _ => assert!(false),
        }


        let a = r".fini_array     :{}";
        match sections(a) {
            IResult::Done("", v @ _) => {
                //assert_eq!(v.len(), 8);
            }
            r @ _ => panic!("{:?}", r),
        }
    }
}
