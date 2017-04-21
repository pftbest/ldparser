use statements::Statement;
use statements::statement;
use memory::Region;

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

named!(root_item<&str, RootItem>, alt_complete!(
    statement_item
));

named!(pub parse<&str, Vec<RootItem>>, many1!(
    root_item
));
