#pragma once
#include <stdint.h>

/// Resource can be read
const uint64_t CAPABILITY_READ = 1 << 0;

/// Resource can be written to
const uint64_t CAPABILITY_WRITE = 1 << 1;

/// Resource contains an executable section
const uint64_t CAPABILITY_EXECUTE = 1 << 2;

/// Resource can be split into subresources
const uint64_t CAPABILITY_SPLIT = 1 << 3;

/// A new derived resource can be created from this resource
const uint64_t CAPABILITY_DERIVE = 1 << 4;

/// This resource right can be revoked
const uint64_t CAPABILITY_REVOKE = 1 << 5;

struct capability_handle {
	uint64_t generation_id;
	uint64_t arena_id;
}__attribute__((packed));

/// Struct for physical resource capabilities
struct resource_capability {
	/// Permission field
	uint64_t permissions;

	/// Pointer to the physical address of the resource
	uint64_t resource;

	/// Physical size of the resource
	uint64_t size;

	/// Randomly generated generation id
	uint64_t genid;

	/// Atomic counter of all capabilities derived from this one
	uint64_t derived_refs;

	/// Atomic counter of all the virtual capabilities tied to this physical capability
	uint64_t virtual_refs;
}__attribute__((aligned(64)));

/// Struct for virtual resource capabilities, pointing also to their physical counterpart
struct virtual_capability {
	/// Permission field
	uint64_t permissions;

	/// Pointer to the virtual address of the resource
	uint64_t resource;
	
	/// Virtual size of the resource
	uint64_t size;

	/// Randomly generated generation id
	uint64_t genid;

	/// Handle of the parent capability
	struct capability_handle rescap;
}__attribute__((aligned(64)));

/// Strut defining the header for a physical resource capability arena
struct resource_capability_arena {
	/// Randomly generated arena id
	uint64_t arenaid;

	/// Total number of slots in the arena
	uint32_t slots;

	/// Total number of free slots in the arena
	uint32_t slots_free;

	/// Self referential capability to the arena itself
	struct resource_capability arena_cap;

	/// Remaining capabilities
	struct resource_capability res_cap[];
}__attribute__((aligned(4096)));

/// Strut defining the header for a virtual resource capability arena
struct virtual_capability_arena {
	/// Randomly generated arena id
	uint64_t arenaid;

	/// Total number of slots in the arena
	uint32_t slots;

	/// Total number of free slots in the arena
	uint32_t slots_free;

	/// Self referential capability to the arena itself
	struct resource_capability arena_cap;
	
	/// Remaining capabilities
	struct virtual_capability virt_cap[];
}__attribute__((aligned(4096)));

struct general_regs {
	uint64_t stack_pointer;
	uint64_t instruction_pointer;
	uint64_t top_level_paging;

#if defined(__x86_64__)
#elif defined(__aarch64__)
#endif
}__attribute__((packed));

/// Struct for a kernel level representation of a task
struct kernel_execution_context {
	struct general_regs regs;

	uint64_t stack_base;
	uint64_t stack_top;

	uint64_t kernel_stack_base;
	uint64_t kernel_stack_top;

	uint64_t interrupt_stack_base;
	uint64_t interrupt_stack_top;

	struct resource_capability_arena *root_resource_capability_arena;
	struct virtual_capability_arena *root_virtual_capability_arena;
}__attribute__((aligned(64)));

/// Valid signature that the bootloader registers as the first field in the boot_loader_data_t structure
/// Packed in a 64-bit integer LE
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

/// Struct for defining executables loaded by strap, which must bep either phisically or virtually contiguous
struct boot_executable_info {
	/// Physical base of the executable
	uint64_t physical_base;

	/// Viertual mapped base of the executable
	uint64_t virtual_base;

	/// Entry point of the executable
	uint64_t entry;

	/// Top level page table structure for the executable
	uint64_t virtual_space;

	/// Page counts
	struct page_counts {
		/// Text page count (RE)
		uint32_t text_pages;

		/// Rodata page count (R)
		uint32_t rodata_pages;

		/// Data page count (RW)
		uint32_t data_pages;

		/// Null page count (_)
		uint32_t null_pages;
	} pages;

	/// CRC32 of the executable
	uint32_t crc32;

	/// Root arenas containing all the caps handed off by the bootloader to the executable
	struct capability_arenas {
		/// Physical capability arena
		uint64_t root_resource_capability_arena;

		/// Virtual capability arena
		uint64_t root_virtual_capability_arena;
	} arenas;
};

/// Struct that wraps all the passed data from strap
struct boot_loader_data {
	/// Signature ('Strapped' in LE)
	uint64_t signature;

	/// Revision of this struct
	uint8_t bootloader_ver;

	/// Padding
	uint8_t padding[3];

	/// Ver 0 fields

	/// Information about the loaded kernel exdecutable
	struct boot_executable_info kernel_executable;

	/// Information about the object manager executable
	struct boot_executable_info objman_executable;
}__attribute__((aligned(4096)));
