# Bare Rusty Metal

Testing Bare Metal Rust, written to be run from the U-Boot bootloader on the ARM Cortex-A9. Target platform is the Terasic DE-10 Nano.

## Documentation

Read the [The Embedded Rust Book](https://docs.rust-embedded.org/book/). Watch [BAREMETAL RUST Runs on EVERYTHING](https://youtu.be/jZT8APrzvc4) by Low Level Learning.

To understand peripherals, refer to the `Cyclone V Hard Processor System Technical Reference Manual` and also the `Cyclone V HPS Register Address Map and Definitions` which can both be found online.

## Dependencies

To build the tool you need:
- Host PC with Rust and Cargo installed
- A cross compiler `rustup target add armv7a-none-eabi`

## Build the tool

Run the `build.sh` script to build the tool using the `linker.ld` file. It will also convert the elf file to a raw binary file `ke`.

```
./build.sh
```

## How to use the tool