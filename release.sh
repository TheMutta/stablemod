#!/bin/sh
set -xe

cargo +nightly -Z unstable-options -C hal build --release --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C hal build --release --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal build --release --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal build --release --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C hal build --release --target aarch64-unknown-none && \
cargo +nightly -Zjson-target-spec -Z unstable-options -C hal build --release --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C strap build --release --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C strap build --release --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap build --release --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap build --release --target aarch64-unknown-uefi && \
cargo +nightly -Zjson-target-spec -Z unstable-options -C kernel build --release --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C kernel build --release --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C kernel build --release --target aarch64-unknown-none && \
cargo +nightly -Z unstable-options -C butler build --release --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C butler build --release --target aarch64-unknown-none
