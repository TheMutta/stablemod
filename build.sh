#!/bin/sh
set -xe

cargo +nightly -Z unstable-options -C hal build --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C hal build --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal build --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal build --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C hal build --target aarch64-unknown-none && \
#cargo +nightly -Zjson-target-spec -Z unstable-options -C hal build --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C strap build --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C strap build --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap build --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap build --target aarch64-unknown-uefi && \
#cargo +nightly -Zjson-target-spec -Z unstable-options -C kernel build --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C kernel build --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C kernel build --target aarch64-unknown-none && \
cargo +nightly -Z unstable-options -C butler build --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C butler build --target aarch64-unknown-none
