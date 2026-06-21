// mcirok hal
// stablemod objectmanager
// userspace subsystems

#include "../abi.h"

#include <stdbool.h>

#include <stdint.h>

static inline void outb(uint16_t port, uint8_t val) {
    __asm__ __volatile__ ("outb %0, %1" : : "a"(val), "Nd"(port));
}

static inline uint8_t inb(uint16_t port) {
    uint8_t val;
    __asm__ __volatile__ ("inb %1, %0" : "=a"(val) : "Nd"(port));
    return val;
}

#define COM1 0x3F8

void init_serial() {
    outb(COM1 + 1, 0x00);    // Disable all interrupts
    outb(COM1 + 3, 0x80);    // Enable DLAB (set baud rate divisor)
    outb(COM1 + 0, 0x03);    // Set divisor to 3 (38400 baud)
    outb(COM1 + 1, 0x00);    // (hi byte)
    outb(COM1 + 3, 0x03);    // 8 bits, no parity, one stop bit
    outb(COM1 + 2, 0xC7);    // Enable FIFO, clear them, 14-byte threshold
    outb(COM1 + 4, 0x0B);    // IRQs enabled, RTS/DSR set
}

void write_serial(char c) {
    while ((inb(COM1 + 5) & 0x20) == 0); // Wait for transmit empty
    outb(COM1, c);
}

void print_serial(const char* str) {
    while (*str) {
        write_serial(*str++);
    }
}

void hypervisor_main(struct boot_loader_data *data) {
	print_serial("hello, world\n");

	while(true) {}



	if (data->signature != BOOTLOADER_SIGNATURE) {
		while(true) {}
	}

	struct resource_capability_arena *root_arena = (struct resource_capability_arena*)data->kernel_executable.arenas.root_resource_capability_arena;
	(void)root_arena;



	
}
