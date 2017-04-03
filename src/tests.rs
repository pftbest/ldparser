use nom::IResult;
use commands::script;
use std::fs::{self, File};
use std::io::Read;

#[test]
fn run_tests() {
    for entry in fs::read_dir("tests").unwrap() {
        let path = entry.unwrap().path();
        println!("testing: {:?}", path);
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        match script(&contents) {
            IResult::Done("", v @ _) => {
                assert!(v.len() != 0);
                println!("OK");
            }
            r @ _ => panic!("{:?}", r),
        }
    }
}
