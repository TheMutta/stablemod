#!/bin/sh
set -xe

cargo +nightly -Z unstable-options -C hal clean --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C hal clean --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal clean --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C hal clean --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C hal clean --target aarch64-unknown-none && \
cargo +nightly -Zjson-target-spec -Z unstable-options -C hal clean --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C strap clean --target x86_64-unknown-linux-musl && \
cargo +nightly -Z unstable-options -C strap clean --target x86_64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap clean --target aarch64-unknown-uefi && \
cargo +nightly -Z unstable-options -C strap clean --target aarch64-unknown-uefi && \
cargo +nightly -Zjson-target-spec -Z unstable-options -C kernel clean --target ../x86_64-unknown-linux-stablemod.json && \
cargo +nightly -Z unstable-options -C kernel clean --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C kernel clean --target aarch64-unknown-none && \
cargo +nightly -Z unstable-options -C butler clean --target x86_64-unknown-none && \
cargo +nightly -Z unstable-options -C butler clean --target aarch64-unknown-none
