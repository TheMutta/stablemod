#!/bin/bash

set -ex

ARCH=${1:-x86_64-unknown-linux}

mkdir -p ./testing

mkdir -p ./testing/esp/efi/boot/
cp ./strap/config_uefi_example.toml ./testing/esp/config.toml
cp ./kernel/target/x86_64-unknown-none/debug/kernel ./testing/esp/kernel.x86_64
cp ./butler/target/x86_64-unknown-none/debug/butler ./testing/esp/butler.x86_64
cp ./strap/target/x86_64-unknown-uefi/debug/strap.efi ./testing/esp/efi/boot/bootx64.efi

mkdir -p ./testing/linux
cp ./strap/config_linux_example.toml ./testing/linux/config.toml
cp ./strap/target/x86_64-unknown-linux-musl/debug/strap ./testing/linux/strap.x86_64
cp ./kernel/target/x86_64-unknown-linux-stablemod/debug/kernel ./testing/linux/kernel.x86_64
cp ./butler/target/x86_64-unknown-none/debug/butler ./testing/linux/butler.x86_64


if [ ! -d ./testing/bios/efi ]; then
	mkdir -p ./testing/bios/efi
	wget -O ./testing/bios/efi/ovmf.tar.xz https://github.com/rust-osdev/ovmf-prebuilt/releases/download/edk2-stable202508-r1/edk2-stable202508-r1-bin.tar.xz
	tar -xf ./testing/bios/efi/ovmf.tar.xz --strip-components=1 -C ./testing/bios/efi/
fi

case $ARCH in
	"x86_64-unknown-linux")
		cd testing/linux/
		./strap.x86_64
		;;
	"x86_64-unknown-uefi")
		qemu-system-x86_64 \
			-machine q35 \
			-m 256M \
			-cpu max \
			-smp 6 \
			-drive if=pflash,format=raw,readonly=on,file=./testing/bios/efi/x64/code.fd \
			-drive if=pflash,format=raw,file=./testing/bios/efi/x64/vars.fd \
			-drive format=raw,file=fat:rw:testing/esp \
			-device virtio-gpu-pci \
			-display none \
			-serial stdio			
		;;
	"aarch64-unknown-uefi")
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
	"riscv64-unknown-uefi")
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


