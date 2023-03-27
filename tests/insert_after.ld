SECTIONS
{
  .got : { *(.got.plt) *(.igot.plt) *(.got) *(.igot) }
}
INSERT AFTER .text;
