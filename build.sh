#!/bin/bash

cargo rustc -- -C link-arg=--script=./linker.ld -C target-cpu=cortex-a9 -C target-feature=+neon,+v7,+vfp3

arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rusty_bare_metal ./ke