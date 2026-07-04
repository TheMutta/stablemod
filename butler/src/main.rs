#![no_std]
#![no_main]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

extern crate alloc;

use core::{alloc::{GlobalAlloc, Layout}, panic::PanicInfo, ptr::null_mut};

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

use hal::syscall;
use hal::log::HalLogger;

use kernel::CapabilityHandle;
use kernel::ResourceCapabilityArena;

#[unsafe(no_mangle)]
extern "C" fn _start(arg0: u64, arg1: u64) {
    HalLogger::init();

    log::info!("Hello, world!");
    log::info!("Butler is alive!");

    let arena_handle = CapabilityHandle::new(arg0, arg1);
    let cap_handle = CapabilityHandle::new(arg0, arg1);

    let buf = [0u8; 4096];

    log::info!("Read!");
    let res = syscall!(
        kernel::SYS_CAP_READ,
        &arena_handle as *const CapabilityHandle as usize,
        &cap_handle as *const CapabilityHandle as usize,
        0,
        &buf as *const u8 as *mut u8 as usize,
        128
    );

    if res == 0 {
        log::info!("Valid read!");

        let arena = unsafe { core::mem::transmute::<[u8; 4096], ResourceCapabilityArena>(buf) };

        if arena.arenaid == arg1 {
            log::info!("read arena id and real arena id match!");
            log::info!("arena size: {}", arena.arena_cap.size);
        }
    }


    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
