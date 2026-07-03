use uefi::prelude::*;
use uefi::CString16;


use alloc::vec::Vec;


#[inline]
pub fn efi_allocate_pages(page_count: usize) -> NonNull<u8> {
     uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate pages")
}

#[inline]
fn efi_str_to_cstring16(s: &str) -> CString16 {
    CString16::try_from(s).unwrap()
}

#[inline]
fn efi_load_file(path: CString16) -> uefi::fs::FileSystemResult<Vec<u8>> {
    let image_handle = uefi::boot::image_handle();
    let image_file_system = uefi::boot::get_image_file_system(image_handle).expect("couldnt get image file system");
    let mut image_file_system = uefi::fs::FileSystem::new(image_file_system);

    image_file_system.read(path.as_ref())
}


use core::ptr::NonNull;
use crate::config::StrapConfig;
use alloc::boxed::Box;
use hal::memory::*;
use hal::log::HalLogger;
use hal::rng::HalRngGenerator;
use hal::paging::{HalPageHierarchy, HalPageFlags};
use core::num::NonZero;

use uefi::system::with_config_table;
use uefi::table::cfg::ConfigTableEntry;
use uefi::println;
use uefi::proto::rng::*;

use kernel::ResourceCapability;
use kernel::ResourceCapabilityArena;

#[entry]
fn efi_main() -> Status { 
    HalLogger::init();
    uefi::helpers::init().unwrap();

    let rng_generator = HalRngGenerator::new();

    let uefi_allocator = Box::leak(Box::new(HalUefiFrameAllocator::new()));
    let frame_allocator = HalFrameAllocatorWrapper::new(uefi_allocator);
    let mut page_hierarchy = HalPageHierarchy::init({
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, 1).unwrap().as_ptr();
        unsafe {
            page.write_bytes(0x00, 4096);
        }
        page as u64
     }, frame_allocator.clone());

    let mut bootloader_info: NonNull<crate::c_abi::boot_loader_data> = {
        let page_count = size_of::<crate::c_abi::boot_loader_data>() / uefi::boot::PAGE_SIZE;
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate bootloader data");
    
        page_hierarchy.mapping(page.as_ptr() as u64, page.as_ptr() as u64, HalPageFlags::R, page_count * 4096);

        unsafe {
            let ptr = NonNull::new_unchecked(page.as_ptr() as *mut crate::c_abi::boot_loader_data);
            ptr.write_bytes(0x00, 1);

            ptr
        }
    };


    match efi_load_file(cstr16!("config.toml").into()) {
        Ok(config) => {
            let config: StrapConfig = toml::from_slice(config.as_slice()).expect("could not parse config");
            log::info!("config: {:#?}", config);
        },
        Err(err) => {
            log::warn!("config not found!");
        },
    }


    let kernel = efi_load_file(
        if cfg!(target_arch = "x86_64") {
            cstr16!("kernel.x86_64").into()
        } else if cfg!(target_arch = "aarch64") {
            cstr16!("kernel.aarch64").into()
        } else {
            cstr16!("kernel.unknown").into()
        }
    ).expect("could not load kernel");
    let objman = efi_load_file(
        if cfg!(target_arch = "x86_64") {
            cstr16!("butler.x86_64").into()
        } else if cfg!(target_arch = "aarch64") {
            cstr16!("butler.aarch64").into()
        } else {
            cstr16!("butler.unknown").into()
        }
    ).expect("could not load objman");
    let kernel = crate::elf::elf_parse_file(&mut page_hierarchy, &frame_allocator, kernel).expect("could not parse kernel");
    let objman = crate::elf::elf_parse_file(&mut page_hierarchy, &frame_allocator, objman).expect("could not parse objman");

    unsafe {
        core::ptr::write(&mut bootloader_info.as_mut().kernel_executable, kernel);
        core::ptr::write(&mut bootloader_info.as_mut().objman_executable, objman);
    }

    with_config_table(|slice| {
        for i in slice {
            match i.guid {
                ConfigTableEntry::ACPI_GUID => {
                    println!("Found ACPI1 at 0x{:x}", i.address as u64);

                    unsafe {
                        println!("{}{}{}{}{}{}{}{}",
                            *(i.address as *const u8) as char,
                            *((i.address as *const u8).add(1)) as char,
                            *((i.address as *const u8).add(2)) as char,
                            *((i.address as *const u8).add(3)) as char,
                            *((i.address as *const u8).add(4)) as char,
                            *((i.address as *const u8).add(5)) as char,
                            *((i.address as *const u8).add(6)) as char,
                            *((i.address as *const u8).add(7)) as char,
                        );
                    }
                },
                ConfigTableEntry::ACPI2_GUID => {
                    println!("Found ACPI2 at 0x{:x}", i.address as u64);

                    unsafe {
                        println!("{}{}{}{}{}{}{}{}",
                            *(i.address as *const u8) as char,
                            *((i.address as *const u8).add(1)) as char,
                            *((i.address as *const u8).add(2)) as char,
                            *((i.address as *const u8).add(3)) as char,
                            *((i.address as *const u8).add(4)) as char,
                            *((i.address as *const u8).add(5)) as char,
                            *((i.address as *const u8).add(6)) as char,
                            *((i.address as *const u8).add(7)) as char,
                        );
                    }
                },
                guid => {
                    println!("Found {}", guid);
                },
            }
        }
    });




    let mut root_capability_arena: NonNull<ResourceCapabilityArena> = {
        let page_count = 4; // TODO
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate root arena");

        let addr = page.as_ptr() as *mut ResourceCapabilityArena;
        unsafe { 
            bootloader_info.as_mut().kernel_executable.arenas.root_resource_capability_arena = addr as u64;

            let ptr = NonNull::new_unchecked(addr);
            ptr.write_bytes(0x00, page_count);

            // TODO: fill rng data
            let mut rng_data = [0u64; 2];
            rng_generator.generate_rng(&mut rng_data);

            let arenaid = rng_data[0];
            let slots = 0;
            let slots_free = 0;

            let permissions = 0;
            let resource = ptr.as_ptr() as u64;
            let size = page_count as u64 * 4096;
            let genid = rng_data[1];
            let derived_refs = 0;
            let virtual_refs = 0;
            let arena_cap = ResourceCapability::new(permissions, resource, size, genid, derived_refs, virtual_refs);

            core::ptr::write(ptr.as_ptr(), ResourceCapabilityArena::new(arenaid, slots, slots_free, arena_cap));

            log::info!("Arena: {:#?}", *ptr.as_ptr());

            ptr
        }
    };


    use uefi::mem::memory_map::MemoryMap;
    use uefi::mem::memory_map::MemoryMapMut;
    let mut memory_map = uefi::boot::memory_map(uefi::boot::MemoryType::LOADER_DATA).unwrap();
    memory_map.sort();
    for entry in memory_map.entries() {
    }

    unsafe { bootloader_info.as_mut().signature = crate::c_abi::BOOTLOADER_SIGNATURE; }

    log::info!("bootloader data initialized: {}", unsafe {
        let sig_ptr = &bootloader_info.as_ref().signature as *const u64 as *const u8;
        core::str::from_utf8(core::slice::from_raw_parts(sig_ptr, 8)).unwrap()
    });

    let stack_bottom = frame_allocator.alloc_frames(NonZero::new(16).unwrap()).expect("alloced_frames").get();
    page_hierarchy.mapping(stack_bottom, stack_bottom, HalPageFlags::RW, 16* 4096);
    let stack_top = stack_bottom + 16 * 4096;

    log::info!("stack top {:X}", stack_top);

    #[cfg(target_arch = "x86_64")]
    unsafe {
        let aligned_stack = stack_top & !0xF;
        let absolute_entry = kernel.entry;
        log::info!("jumping to {:X}:{:X}", absolute_entry, aligned_stack);
        let _ = uefi::boot::exit_boot_services(None);
        core::arch::asm!(
            "mov rdi, {bootloader_info}",
            "mov rsp, {stack_ptr}",
            "jmp {entry}",
            stack_ptr = in(reg) aligned_stack,
            entry = in(reg) absolute_entry,
            bootloader_info = in(reg) bootloader_info.as_ptr(),
            options(noreturn)
        );
    }

    #[cfg(target_arch = "aarch64")]
    unsafe {
        let aligned_stack = stack_top & !0xF;
        let absolute_entry = kernel.entry;
        log::info!("jumping to {:X}:{:X}", absolute_entry, aligned_stack);
        let _ = uefi::boot::exit_boot_services(None);
        core::arch::asm!(
            "mov sp, {stack_ptr}",
            "br {entry}",
            stack_ptr = in(reg) aligned_stack,
            entry = in(reg) absolute_entry,
            options(noreturn)
        );
    }

    loop {}


    Status::SUCCESS

}
