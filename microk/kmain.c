// mcirok hal
// stablemod objectmanager
// userspace subsystems

#include "../abi.h"

#include <stdbool.h>

void hypervisor_main(boot_loader_data_t *data) {
	if (data->signature != BOOTLOADER_SIGNATURE) {
		while(true) {}
	}

	resource_capability_arena_t *root_arena = (resource_capability_arena_t*)data->kernel_executable.root_resource_capability_arena;
	(void)root_arena;



	
}
