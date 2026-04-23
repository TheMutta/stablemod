#!/bin/sh
set -xe

#cargo -Z unstable-options -C stablemod-config build
cargo -Z unstable-options -C stablemod-bootloader build
cargo -Z unstable-options -C stablemod-kernel build

mkdir -p build
