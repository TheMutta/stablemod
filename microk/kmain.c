// mcirok hal
// stablemod objectmanager
// userspace subsystems

#include "../abi.h"

#include <stdbool.h>

void hypervisor_main(struct boot_loader_data *data) {
	if (data->signature != BOOTLOADER_SIGNATURE) {
		while(true) {}
	}

	struct resource_capability_arena *root_arena = (struct resource_capability_arena*)data->kernel_executable.arenas.root_resource_capability_arena;
	(void)root_arena;



	
}
