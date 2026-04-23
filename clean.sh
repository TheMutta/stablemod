#!/bin/sh
set -xe

cargo -Z unstable-options -C stablemod-bootloader clean
cargo -Z unstable-options -C stablemod-kernel clean

mkdir -p build
