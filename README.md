# Bare Rusty Metal

Testing Bare Metal Rust, written to be run from the U-Boot bootloader on the ARM Cortex-A9. Target platform is the Terasic DE-10 Nano.

## Documentation

Read the [The Embedded Rust Book](https://docs.rust-embedded.org/book/). Watch [BAREMETAL RUST Runs on EVERYTHING](https://youtu.be/jZT8APrzvc4) by Low Level Learning.

For peripherals refer to the [Cyclone V Hard Processor System Technical Reference Manual](https://www.intel.com/content/www/us/en/docs/programmable/683126/21-2/hard-processor-system-technical-reference.html) and also the [Cyclone V HPS Register Address Map and Definitions](https://www.intel.com/content/www/us/en/programmable/hps/cyclone-v/hps.html) which can both be found online.

## Dependencies

To build the tool you need:
- Host PC with Rust and Cargo installed
- A cross compiler `rustup target add armv7a-none-eabi`
- A device with an ARM Cortex-A9 (or similar) running U-Boot (tested with SPL 2023.04)

## Build the tool

Run the `build.sh` script to build the tool using the `linker.ld` file. It will also convert the elf file to a raw binary file `bin`.

```
./build.sh
```

If you need to make changes to the linker.ld file, be sure to delete the `target` folder, otherwise the compiler might skip recompiling.

## How to use the tool

You need a way to copy the built raw binary file `bin` from your host PC to the ARM. Options include JTAG, TFTP, serial UART custom protocols or using the SD card.

In my case I am using SSH to copy the file on the target device SD card when it runs buildroot Linux. After that I am rebooting the ARM to get back into the U-boot. Not exactly sensible considering the final product is not supposed to have a Linux running but o well. The easiest but also most annoying alternative is to simply copy the file over manually with an SD card reader.

Modify and then run the `cp_to_arm.sh` script.

```
./cp_to_arm.sh
```

Boot into the U-Boot. To copy the binary from the SD card into the RAM at address 0x0100_0000, and then run from this address, run the following commands:

```
fatload mmc 0:1 0x01000000 bin
go 0x01000000
```

It should say something like:

```
## Starting application at 0x01000000 ...
## Application terminated
```