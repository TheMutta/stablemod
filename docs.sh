#!/bin/sh
set -xe

cargo -Z unstable-options -C strap doc
#make -C microk all
cargo -Z unstable-options -C butler doc
