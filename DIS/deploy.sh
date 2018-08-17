#!/bin/bash
scp -p ./firmware/target/armv7-unknown-linux-gnueabihf/debug/firmware martinz@dis-proto:~/firmware/firmware
scp -pr ./data martinz@dis-proto:~/firmware/data
