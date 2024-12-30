#!/bin/sh

echo "qemu-system-i386 -drive format=raw,file=$1"
qemu-system-i386 -drive format=raw,file="$1"