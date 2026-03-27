#!/bin/bash

ARCH=${1:-x86_64}

mkdir -p ./testing

cargo -Z unstable-options -C stablemod-bootloader build
cargo -Z unstable-options -C stablemod-kernel build

mkdir -p ./testing/esp/efi/boot/
cp ./stablemod-kernel/target/x86_64-unknown-none/debug/stablemod-kernel ./testing/esp/stablemod.x64
cp ./stablemod-kernel/target/aarch64-unknown-none/debug/stablemod-kernel ./testing/esp/stablemod.aarch64
cp ./stablemod-bootloader/target/x86_64-unknown-uefi/debug/stablemod-bootloader.efi ./testing/esp/efi/boot/bootx64.efi
cp ./stablemod-bootloader/target/aarch64-unknown-uefi/debug/stablemod-bootloader.efi ./testing/esp/efi/boot/bootaa64.efi

if [ ! -d ./testing/bios/efi ]; then
	mkdir -p ./testing/bios/efi
	wget -O ./testing/bios/efi/ovmf.tar.xz https://github.com/rust-osdev/ovmf-prebuilt/releases/download/edk2-stable202508-r1/edk2-stable202508-r1-bin.tar.xz
	tar -xf ./testing/bios/efi/ovmf.tar.xz --strip-components=1 -C ./testing/bios/efi/
fi

case $ARCH in
	"x86_64")
		qemu-system-x86_64 \
			-machine q35 \
			-m 128M \
			-cpu max \
			-smp 6 \
			-drive if=pflash,format=raw,readonly=on,file=./testing/bios/efi/x64/code.fd \
			-drive if=pflash,format=raw,file=./testing/bios/efi/x64/vars.fd \
			-drive format=raw,file=fat:rw:testing/esp \
			-device virtio-gpu-pci \
			-serial stdio			
		;;
	"aarch64")
		qemu-system-aarch64 \
			-machine virt \
			-m 128M \
			-cpu max \
			-smp 6 \
			-drive if=pflash,format=raw,readonly=on,file=./testing/bios/efi/aarch64/code.fd \
			-drive format=raw,file=fat:rw:testing/esp \
			-device virtio-gpu-pci \
			-serial stdio
		;;
	"riscv64")
		qemu-system-riscv64 \
			-machine virt \
			-m 128M \
			-cpu max \
			-smp 6 \
			-drive if=pflash,format=raw,readonly=on,file=./testing/bios/efi/riscv64/code.fd \
			-drive format=raw,file=fat:rw:testing/esp \
			-device virtio-gpu-pci \
			-serial stdio
		;;
esac


