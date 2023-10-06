ENTRY(_start)

MEMORY
{
  RAM (wx) : ORIGIN = 0x01000000, LENGTH = 128K
}

SECTIONS
{
  .text :
  {
      *(.text._start)
      *(.text)
  } > RAM

  .rodata :
  {
  *(.rodata)
  } > RAM

  .data :
  {
  *(.data)
  } > RAM

  .bss :
  {
    *(.bss)
  } > RAM

  .ARM.exidx :
  {
    *(.ARM.exidx*)
  } > RAM
}
