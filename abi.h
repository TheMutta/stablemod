#pragma once
#include <stdint.h>

typedef enum {
	CAPABILITY_READ = 1 << 0,
	CAPABILITY_WRITE = 1 << 1,
	CAPABILITY_EXECUTE = 1 << 2,
	CAPABILITY_SPLIT = 1 << 3,
	CAPABILITY_DERIVE = 1 << 4,
	CAPABILITY_REVOKE = 1 << 5,
} resource_capability_permission_t;


/// Struct for physical resource capabilities
typedef struct {
	uint64_t permissions;

	uint64_t resource;
	uint64_t size;

	uint64_t genid;

	uint64_t virtual_refs;
}__attribute__((aligned(64))) resource_capability_t;

/// Struct for virtual resource capabilities, pointing also to their physical counterpart
typedef struct {
	uint64_t permissions;

	uint64_t ptr;
	uint64_t size;

	uint64_t genid;

	uint64_t rescap_arenaid;
	uint64_t rescap_genid;
}__attribute__((aligned(64))) virtual_capability_t;

typedef struct {
	uint64_t arenaid;
	uint64_t slots;
	resource_capability_t arena_cap;
	resource_capability_t res_cap[];
}__attribute__((aligned(4096))) resource_capability_arena_t;

typedef struct {
	uint64_t arenaid;
	uint64_t slots;
	resource_capability_t arena_cap;
	virtual_capability_t virt_cap[];
}__attribute__((aligned(4096))) virtual_capability_arena_t;





const uint64_t BOOTLOADER_SIGNATURE = ( \
	((uint64_t)'S' << 0) | \
	((uint64_t)'t' << 8) | \
	((uint64_t)'r' << 16) | \
	((uint64_t)'a' << 24) | \
	((uint64_t)'p' << 32) | \
	((uint64_t)'p' << 40) | \
	((uint64_t)'e' << 48) | \
	((uint64_t)'d' << 56) \
	);

typedef struct {
	uint64_t physical_base;
	uint64_t virtual_base;

	uint64_t entry;
	uint64_t virtual_space;

	uint32_t text_pages;
	uint32_t rodata_pages;
	uint32_t data_pages;
	uint32_t crc32;

	/// Root arenas containing all the caps handed off by the bootloader to the executable
	uint64_t root_resource_capability_arena;
	uint64_t root_virtual_capability_arena;
} boot_executable_info_t;

typedef struct {
	// Signature
	uint64_t signature;

	/// Revision of this struct
	uint8_t bootloader_ver;
	uint8_t padding[3];


	/// Ver 0 
	boot_executable_info_t kernel_executable;
	boot_executable_info_t objman_executable;
}__attribute__((aligned(4096))) boot_loader_data_t;
