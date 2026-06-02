#!/bin/sh
set -xe

cargo -Z unstable-options -C butler clean
make -C microk clean
cargo -Z unstable-options -C strap clean

mkdir -p build
