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
#[entry]
fn efi_main() -> Status { 
    uefi::helpers::init().unwrap();

    let mut bootloader_info: NonNull<crate::c_abi::boot_loader_data> = {
        let page_count = size_of::<crate::c_abi::boot_loader_data>() / uefi::boot::PAGE_SIZE;
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate bootloader data");

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

    {
        let kernel = efi_load_file(cstr16!("kernel.x86_64").into()).expect("could not load kernel");
        let objman = efi_load_file(cstr16!("butler.x86_64").into()).expect("could not load objman");
        let kernel = crate::elf::elf_parse_file(kernel).expect("could not parse kernel");
        let objman = crate::elf::elf_parse_file(objman).expect("could not parse objman");

        unsafe {
            core::ptr::write(&mut bootloader_info.as_mut().kernel_executable, kernel);
            core::ptr::write(&mut bootloader_info.as_mut().objman_executable, objman);
        }
    }

    let mut root_capability_arena: NonNull<crate::c_abi::resource_capability_arena> = {
        let page_count = 4; // TODO
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate root arena");

        let addr = page.as_ptr() as *mut crate::c_abi::resource_capability_arena; 
        unsafe { 
            bootloader_info.as_mut().kernel_executable.arenas.root_resource_capability_arena = addr as u64;

            let ptr = NonNull::new_unchecked(addr);
            ptr.write_bytes(0x00, page_count);

            ptr
        }
    };

    unsafe { bootloader_info.as_mut().signature = crate::c_abi::BOOTLOADER_SIGNATURE; }

    log::info!("bootloader data initialized: {}", unsafe {
        let sig_ptr = &bootloader_info.as_ref().signature as *const u64 as *const u8;
        core::str::from_utf8(core::slice::from_raw_parts(sig_ptr, 8)).unwrap()
    });

    loop {}

    Status::SUCCESS

}
