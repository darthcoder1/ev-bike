#!/bin/bash

USER=root
DEVICE_NAME=dis-proto
TARGET_PATH=/opt/firmware

ls -la

scp -p ./target/armv7-unknown-linux-gnueabihf/debug/firmware $USER@$DEVICE_NAME:$TARGET_PATH/firmware
scp -pr ./../data $USER@$DEVICE_NAME:$TARGET_PATH/data
