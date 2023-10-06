/* Simple linker script for Cortex-A9 bare-metal program */

ENTRY(_start)

MEMORY
{
    /* Define the memory regions for the Cortex-A9 */
    RAM (wx) : ORIGIN = 0x01000000, LENGTH = 128K
}

SECTIONS
{
    /* The reset vector at the beginning of RAM 
    .vectors :
    {
        *(.vectors)
    } > RAM*/

    /* The .text section contains your code */
    .text :
    {
        *(.text._start)
        *(.text)
    } > RAM

    /* The .rodata section contains read-only data */
    .rodata :
    {
        *(.rodata)
    } > RAM

    /* The .data section contains initialized data */
    .data :
    {
        *(.data)
    } > RAM

    /* The .bss section contains uninitialized data */
    .bss :
    {
        *(.bss)
    } > RAM

     .ARM.exidx :
    {
        *(.ARM.exidx*)
    } > RAM
}
