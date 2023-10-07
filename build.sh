#!/bin/bash

cargo build

arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rusty_bare_metal ./bin