#![no_std]
#![no_main]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

use core::{panic::PanicInfo, ptr::NonNull};

use uefi::prelude::*;

extern crate alloc;

#[entry]
fn efi_main() -> Status { 
    uefi::helpers::init().unwrap();

    let image_handle = uefi::boot::image_handle();
    let image_file_system = uefi::boot::get_image_file_system(image_handle).expect("couldnt get image file system");
    let mut image_file_system = uefi::fs::FileSystem::new(image_file_system);
    let path = uefi::CString16::try_from("/config.toml").unwrap();
    image_file_system.read(path.as_ref());

    let mut bootloader_info: NonNull<c_abi::boot_loader_data_t> = {
        let page_count = size_of::<c_abi::boot_loader_data_t>() / uefi::boot::PAGE_SIZE;
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate bootloader data");

        unsafe {
            let ptr = NonNull::new_unchecked(page.as_ptr() as *mut c_abi::boot_loader_data_t);
            ptr.write_bytes(0x00, 1);

            ptr
        }
    };


    let mut root_capability_arena: NonNull<c_abi::resource_capability_arena_t> = {
        let page_count = 4; // TODO
        let page = uefi::boot::allocate_pages(uefi::boot::AllocateType::AnyPages, uefi::boot::MemoryType::LOADER_DATA, page_count).expect("could not allocate root arena");

        let addr = page.as_ptr() as *mut c_abi::resource_capability_arena_t; 
        unsafe { 
            bootloader_info.as_mut().kernel_executable.root_resource_capability_arena = addr as u64;

            let ptr = NonNull::new_unchecked(addr);
            ptr.write_bytes(0x00, 4);

            ptr
        }
    };

    unsafe { bootloader_info.as_mut().signature = c_abi::BOOTLOADER_SIGNATURE; }

    log::info!("bootloader data initialized: {}", unsafe {
        let sig_ptr = &bootloader_info.as_ref().signature as *const u64 as *const u8;
        core::str::from_utf8(core::slice::from_raw_parts(sig_ptr, 8)).unwrap()
    });

    loop {}

    Status::SUCCESS

}
/*
#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}*/
