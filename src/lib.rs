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
        let text = include_str!("/home/user/Public/rust_on_msp/ldscripts/msp430g2553.ld");
        match script(text) {
            IResult::Done("", v @ _) => {
                assert_eq!(v.len(), 5);
            }
            r @ _ => panic!("{:?}", r),
        }
    }

    // #[test]
    // fn test_real_file_2() {
    //     let text = include_str!("/usr/arm-none-eabi/lib/ldscripts/armelf.x");
    //     match script(text) {
    //         IResult::Done("", v @ _) => {
    //             assert_eq!(v.len(), 5);
    //         }
    //         r @ _ => panic!("{:?}", r),
    //     }
    // }

    #[test]
    fn test_part() {
        let text = r#"

/* Default linker script, for normal executables */
/* Copyright (C) 2014-2017 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */
OUTPUT_FORMAT("elf32-littlearm", "elf32-bigarm",
	      "elf32-littlearm")
OUTPUT_ARCH(arm)
ENTRY(_start)
SEARCH_DIR("=/tmp/jenkins-GCC-6-buildandreg-104_20170216_1487268972/install-native/arm-none-eabi/lib"); SEARCH_DIR("=/usr/local/lib"); SEARCH_DIR("=/lib"); SEARCH_DIR("=/usr/lib");
SECTIONS
{
  /* Read-only sections, merged into text segment: */
  PROVIDE (__executable_start = SEGMENT_START("text-segment", 0x8000)); . = SEGMENT_START("text-segment", 0x8000);
  .interp         : { *(.interp) }
  .note.gnu.build-id : { *(.note.gnu.build-id) }
  .hash           : { *(.hash) }
  .gnu.hash       : { *(.gnu.hash) }
  .dynsym         : { *(.dynsym) }
  .dynstr         : { *(.dynstr) }
  .gnu.version    : { *(.gnu.version) }
  .gnu.version_d  : { *(.gnu.version_d) }
  .gnu.version_r  : { *(.gnu.version_r) }
  .rel.init       : { *(.rel.init) }
  .rela.init      : { *(.rela.init) }
}
    "#;

        match script(text) {
            IResult::Done("", v @ _) => {
                assert_eq!(v.len(), 8);
            }
            r @ _ => panic!("{:?}", r),
        }
    }
}
