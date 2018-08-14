#!/bin/bash

if [ "$1" = "" ]; then
    echo "Usage: prepare_sysroot_from_rpi.sh {rpi_username} {rpi_ipaddr}"
    exit 0
fi

USER_NAME=$1
IP_ADDR=$2

rm -rf $PWD/opt

mkdir -p $PWD/opt/vc/lib
scp -pr $USER_NAME@$IP_ADDR:/opt/vc/lib $PWD/opt/vc/
