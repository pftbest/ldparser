use symbols::symbol_name;
use numbers::number;

#[derive(Debug, PartialEq)]
pub struct Region {
    pub name: String,
    pub origin: u64,
    pub length: u64,
}

named!(attributes<&str, &str>, delimited!(
    tag_s!("("),
    take_until_s!(")"),
    tag_s!(")")
));

named!(origin<&str, &str>, alt_complete!(
    tag_s!("ORIGIN") | tag_s!("org") | tag_s!("o")
));

named!(length<&str, &str>, alt_complete!(
    tag_s!("LENGTH") | tag_s!("len") | tag_s!("l")
));

named!(region<&str, Region>, wsc!(do_parse!(
    name: symbol_name
    >>
    opt!(attributes)
    >>
    tag_s!(":")
    >>
    origin
    >>
    tag_s!("=")
    >>
    org: number
    >>
    tag_s!(",")
    >>
    length
    >>
    tag_s!("=")
    >>
    len: number
    >>
    (Region {
        name: name.into(),
        origin: org,
        length: len
    })
)));

named!(pub regions<&str, Vec<Region>>, many0!(
    region
));

#[cfg(test)]
mod test {
    use nom::IResult;
    use memory::{regions, Region};

    #[test]
    fn test_regions() {
        assert_eq!(regions(r"
            SFR ( rwx )      : ORIGIN = 0x0000, LENGTH = 0x0010 /* END=0x0010, size 16 */
            RAM              : ORIGIN = 0x0200, LENGTH = 0x0200 /* END=0x03FF, size 512 */
            INFOMEM          : ORIGIN = 0x1000, LENGTH = 0x0100 /* END=0x10FF,
             size 256 as 4 64-byte segments */
            ROM              : org = 0x2000, len = 4K
            "),
                   IResult::Done("",
                                 vec![Region {
                                          name: String::from("SFR"),
                                          origin: 0,
                                          length: 16,
                                      },
                                      Region {
                                          name: String::from("RAM"),
                                          origin: 512,
                                          length: 512,
                                      },
                                      Region {
                                          name: String::from("INFOMEM"),
                                          origin: 4096,
                                          length: 256,
                                      },
                                      Region {
                                          name: String::from("ROM"),
                                          origin: 8192,
                                          length: 4096,
                                      }]));
    }
}
