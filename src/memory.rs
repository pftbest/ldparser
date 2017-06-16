use idents::symbol;
use numbers::number;
use whitespace::opt_space;

#[derive(Debug, PartialEq)]
pub struct Region {
    pub name: String,
    pub origin: u64,
    pub length: u64,
}

named!(attributes<&str, &str>, delimited!(
    tag!("("),
    take_until!(")"),
    tag!(")")
));

named!(origin<&str, &str>, alt_complete!(
    tag!("ORIGIN") | tag!("org") | tag!("o")
));

named!(length<&str, &str>, alt_complete!(
    tag!("LENGTH") | tag!("len") | tag!("l")
));

named!(pub region<&str, Region>, do_parse!(
    name: symbol
    >>
    opt_space
    >>
    opt!(attributes)
    >>
    wsc!(tag!(":"))
    >>
    origin
    >>
    wsc!(tag!("="))
    >>
    org: number
    >>
    wsc!(tag!(","))
    >>
    length
    >>
    wsc!(tag!("="))
    >>
    len: number
    >>
    (Region {
        name: name.into(),
        origin: org,
        length: len
    })
));

#[cfg(test)]
mod tests {
    use memory::*;

    #[test]
    fn test_region() {
        assert_done!(
            region("rom (rx)  : ORIGIN = 0, LENGTH = 256K"),
            Region {
                name: "rom".into(),
                origin: 0,
                length: 256 * 1024,
            }
        );
        assert_done!(
            region("ram (!rx) : org = 0x40000000, l = 4M"),
            Region {
                name: "ram".into(),
                origin: 0x40000000,
                length: 4 * 1024 * 1024,
            }
        );
    }
}
