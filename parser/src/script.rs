use statements::Statement;
use statements::statement;
use memory::Region;
use memory::region;

#[derive(Debug, PartialEq)]
pub enum RootItem {
    Statement(Statement),
    Memory { regions: Vec<Region> },
    Sections {},
}

named!(statement_item<&str, RootItem>, map!(
    statement,
    |stmt| RootItem::Statement(stmt)
));

named!(memory_item<&str, RootItem>, do_parse!(
    tag!("MEMORY")
    >>
    wsc!(tag!("{"))
    >>
    regions: wsc!(many1!(
        region
    ))
    >>
    tag!("}")
    >>
    (RootItem::Memory {
        regions: regions
    })
));

named!(root_item<&str, RootItem>, alt_complete!(
    memory_item | statement_item
));

named!(pub parse<&str, Vec<RootItem>>, wsc!(many1!(
    root_item
)));

#[cfg(test)]
mod tests {
    use script::parse;

    #[test]
    fn test_memory() {
        assert_done!(parse(r"
        MEMORY
            {
                rom (rx)  : ORIGIN = 0, LENGTH = 256K
                ram (!rx) : org = 0x40000000, l = 4M
            }
        "));
    }
}
