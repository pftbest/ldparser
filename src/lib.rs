#[macro_use]
extern crate nom;

#[macro_use]
mod whitespace;

mod numbers;
mod symbols;
mod expressions;
mod statements;
mod memory;
mod sections;
mod commands;

pub fn parse(script: &str) {
    commands::script(script).unwrap();
}

#[cfg(test)]
mod test {
    use nom::IResult;
    use commands::script;

    #[test]
    fn test_real_file_1() {
        let text = include_str!("/Users/vadzim/Downloads/rust/rust_on_msp/ldscripts/cc430f5137.ld");
        match script(text) {
            IResult::Done("", v @ _) => {
                assert_eq!(v.len(), 5);
            }
            r @ _ => panic!("{:?}", r),
        }
    }

    #[test]
    fn test_real_file_2() {
        let text = include_str!("/usr/local/Cellar/none-eabi/6.2017.q1-1/arm-none-eabi/lib/ldscripts/armelf.x");
        match script(text) {
            IResult::Done("", v @ _) => {
                assert_eq!(v.len(), 5);
            }
            r @ _ => panic!("{:?}", r),
        }
    }
}
