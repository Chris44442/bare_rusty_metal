#!/bin/bash

cargo build
# cargo build --release

arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rusty_bare_metal ./bin
# arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/release/rusty_bare_metal ./bin