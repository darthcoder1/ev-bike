#!/bin/bash
echo "Building Firmware"

# build the project
~/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf

echo "done" 