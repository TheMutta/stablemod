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

use core::panic::PanicInfo;
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}

use hal::cpu::HalProcessor;
static mut BOOT_PROCESSOR: HalProcessor = HalProcessor::new();

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! { 
    HalLogger::init();

    log::info!("hello, world!");
    log::info!("this is the kernel speaking!");

    unsafe {
        BOOT_PROCESSOR.init();
    }
    
    log::info!("cpu initialised");

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
