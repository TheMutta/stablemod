#!/bin/sh
set -xe

cargo -Z unstable-options -C hal clean --target x86_64-unknown-linux-gnu && \
cargo -Z unstable-options -C hal clean --target x86_64-unknown-uefi && \
cargo -Z unstable-options -C hal clean --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C hal clean --target x86_64-unknown-none && \
cargo -Z unstable-options -C hal clean --target aarch64-unknown-none && \
cargo -Z unstable-options -C strap clean --target x86_64-unknown-linux-gnu && \
cargo -Z unstable-options -C strap clean --target x86_64-unknown-uefi && \
cargo -Z unstable-options -C strap clean --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C strap clean --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C kernel clean --target x86_64-unknown-none && \
cargo -Z unstable-options -C kernel clean --target aarch64-unknown-none && \
cargo -Z unstable-options -C butler clean --target x86_64-unknown-none && \
cargo -Z unstable-options -C butler clean --target aarch64-unknown-none
