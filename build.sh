#!/bin/sh
set -xe

cargo -Z unstable-options -C hal build --target x86_64-unknown-linux-musl && \
cargo -Z unstable-options -C hal build --target x86_64-unknown-uefi && \
cargo -Z unstable-options -C hal build --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C hal build --target x86_64-unknown-none && \
cargo -Z unstable-options -C hal build --target aarch64-unknown-none && \
cargo -Z unstable-options -C strap build --target x86_64-unknown-linux-musl && \
cargo -Z unstable-options -C strap build --target x86_64-unknown-uefi && \
cargo -Z unstable-options -C strap build --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C strap build --target aarch64-unknown-uefi && \
cargo -Z unstable-options -C kernel build --target x86_64-unknown-linux-musl && \
cargo -Z unstable-options -C kernel build --target x86_64-unknown-none && \
cargo -Z unstable-options -C kernel build --target aarch64-unknown-none && \
cargo -Z unstable-options -C butler build --target x86_64-unknown-none && \
cargo -Z unstable-options -C butler build --target aarch64-unknown-none
