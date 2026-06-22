pub fn main() {
    hal::log::init();

    log::info!("hello from strap on linux");

    let ram_size = 128 * 1024 * 1024;
    log::info!("initializing phisical ram of size {}", ram_size);
    let physical_mem_fd = unsafe {
        let name = std::ffi::CString::new("physical_memory").unwrap();
        let fd = libc::memfd_create(name.as_ptr(), 0);
        assert!(fd >= 0, "Failed to create memfd");

        let ram_size: libc::off_t = ram_size;
        let res = libc::ftruncate(fd, ram_size);
        assert!(res == 0, "Failed to allocate memfd space");

        fd
    };

    log::info!("ram initialised at fd {}", physical_mem_fd);

    let mut bootloader_info: crate::c_abi::boot_loader_data;
    let kernel = std::fs::read("kernel.x86_64").expect("could not load kernel");
    let objman = std::fs::read("butler.x86_64").expect("could not load objman");
}
