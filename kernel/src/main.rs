#![no_std]
#![no_main]
#![allow(static_mut_refs)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

use hal::log::HalLogger;
use hal::paging::HalPageHierarchy;
use hal::memory::{HalFrameAllocatorWrapper, HalBareFrameAllocator};

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic_handler(info: &PanicInfo) -> ! {
    log::error!("{}", info);
    loop {}
}

use hal::cpu::HalProcessor;
use hal::cpu::do_userland_jump;
use hal::batch_syscalls;
static mut BOOT_PROCESSOR: HalProcessor = HalProcessor::new();
static mut FRAME_ALLOCATOR: HalBareFrameAllocator = HalBareFrameAllocator::new();

extern "C" fn sys_debug( count: usize) -> usize {
    log::info!("write!");

    0
}

batch_syscalls!(
    sys_0_handler => sys_debug,
);

#[unsafe(no_mangle)]
extern "C" fn _start(bootloader_data: *const crate::c_abi::boot_loader_data) -> ! { 
    HalLogger::init();

    let frame_allocator = unsafe { HalFrameAllocatorWrapper::new(&FRAME_ALLOCATOR) };

    log::info!("hello, world!");
    log::info!("this is the kernel speaking!");

    unsafe {
        BOOT_PROCESSOR.init();
    }
    
    log::info!("cpu initialised");

    let bootloader_data = unsafe { &(*bootloader_data) };
    if bootloader_data.signature != crate::c_abi::BOOTLOADER_SIGNATURE {
        panic!("Invalid bootloader data!");
    }

    log::info!("bootloader data: {:#?}", bootloader_data);

    let page_hierarchy = HalPageHierarchy::init(bootloader_data.kernel_executable.virtual_space, frame_allocator.clone());
    page_hierarchy.switch();

    log::info!("kernel execution finished, handing off!");

    unsafe {
        do_userland_jump(bootloader_data.objman_executable.entry, 0);
    }

    loop {}
}

extern crate alloc;

use core::{alloc::{GlobalAlloc, Layout}, ptr::null_mut};

struct BaseAlloc;

unsafe impl GlobalAlloc for BaseAlloc {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}




#[global_allocator]
static mut GLOBAL_ALLOC: BaseAlloc = BaseAlloc;
