use hal::memory::*;
use alloc::boxed::Box;

use crate::elf::elf_parse_file;

pub fn main() {
    hal::log::init();

    log::info!("hello from strap on linux");

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
            libc::PROT_READ | libc::PROT_WRITE,
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

    let mut bootloader_info: crate::c_abi::boot_loader_data;
    let kernel = std::fs::read("kernel.x86_64").expect("could not load kernel");
    let objman = std::fs::read("butler.x86_64").expect("could not load objman");

    let kernel = elf_parse_file(&frame_allocator, kernel);
    let objman = elf_parse_file(&frame_allocator, objman);
}
