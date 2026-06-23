use hal::memory::*;
use alloc::boxed::Box;

use hal::log::HalLogger;
use crate::elf::elf_parse_file;
use std::num::NonZero;


pub fn main() {
    HalLogger::init();

    log::info!("hello from strap on linux");

    let mut bootloader_info = Box::leak(Box::new(crate::c_abi::boot_loader_data::default()));
    bootloader_info.signature = crate::c_abi::BOOTLOADER_SIGNATURE;

    let ram_size: u64 = 128 * 1024 * 1024;
    log::info!("initializing phisical ram of size {}", ram_size);
    let (physical_mem_fd,physical_memory_offset) = unsafe {
        let name = std::ffi::CString::new("physical_memory").unwrap();
        let fd = libc::memfd_create(name.as_ptr(), 0);
        assert!(fd >= 0, "Failed to create memfd");

        let ram_size: libc::off_t = ram_size as i64;
        let res = libc::ftruncate(fd, ram_size);
        assert!(res == 0, "Failed to allocate memfd space");

        let memfd_virt_addr = libc::mmap(
            std::ptr::null_mut(),
            ram_size as usize,
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_SHARED,
            fd,
            0,
        );
        assert!(memfd_virt_addr != libc::MAP_FAILED);

        (fd, memfd_virt_addr as u64)
    };

    log::info!("ram initialised at fd {} and addr {:X}", physical_mem_fd, physical_memory_offset);

    let memfd_allocator = Box::leak(Box::new(HalMemfdFrameAllocator::new(physical_memory_offset, ram_size)));
    let frame_allocator = HalFrameAllocatorWrapper::new(memfd_allocator);

    let kernel = std::fs::read("kernel.x86_64").expect("could not load kernel");
    let objman = std::fs::read("butler.x86_64").expect("could not load objman");
    let kernel = elf_parse_file(&frame_allocator, kernel).expect("could not parse kernel");
    let objman = elf_parse_file(&frame_allocator, objman).expect("could not parse objman");
    bootloader_info.kernel_executable = kernel;
    bootloader_info.objman_executable = objman;

    let stack_bottom = frame_allocator.alloc_frames(NonZero::new(16).unwrap()).expect("alloced_frames").get();
    let stack_top = stack_bottom + 16 * 4096;

    log::info!("stack top {:X}", stack_top);

    unsafe {
        let aligned_stack = stack_top & !0xF;
        let absolute_entry = kernel.entry;
        log::info!("jumping to {:X}:{:X}", absolute_entry, aligned_stack);
        core::arch::asm!(
            "mov rdi, {bootloader_info}",
            "mov rsp, {stack_ptr}",
            "jmp {entry}",
            stack_ptr = in(reg) aligned_stack,
            entry = in(reg) absolute_entry,
            bootloader_info = in(reg) bootloader_info,
            options(noreturn)
        );
    }
}
