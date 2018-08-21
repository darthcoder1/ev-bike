#!/bin/bash
echo "Building Firmware"

# build the project
echo $USER
echo $PWD

source ~/.bashrc

~/.cargo/bin/cargo build --target=armv7-unknown-linux-gnueabihf

echo "done" 