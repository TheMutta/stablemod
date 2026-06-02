#!/bin/sh
set -xe

cargo -Z unstable-options -C strap build
make -C microk all
cargo -Z unstable-options -C butler build
