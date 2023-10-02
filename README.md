# Bare Rusty Metal

Testing Bare Metal Rust, written to be run from the U-Boot bootloader on the ARM Cortex-A9.

## Documentation

To understand peripherals, refer to the `Cyclone V Hard Processor System Technical Reference Manual` and also the `Cyclone V HPS Register Address Map and Definitions` which can both be found online.

Read the [The Embedded Rust Book](https://docs.rust-embedded.org/book/). Watch [BAREMETAL RUST Runs on EVERYTHING](https://youtu.be/jZT8APrzvc4) by Low Level Learning.

## Dependencies

To build the tool you need:
- Host PC with Rust and Cargo installed
- A cross compiler `rustup target add armv7a-none-eabi`

## Build the tool

```
cargo rustc -- -C link-arg=--script=./linker.ld
```


## How to use the tool