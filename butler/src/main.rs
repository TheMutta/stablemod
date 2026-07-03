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

pub struct LogWriter;

impl core::fmt::Write for LogWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            // Trigger your macro: syscall!(num, arg1, arg2, arg3)
            syscall!(
                0, 
                s.as_ptr() as usize, 
                s.len()
            );
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let _ = write!($crate::LogWriter, $($arg)*);
    }};
}

#[unsafe(no_mangle)]
fn _start() {
    print!("Hello, world!");
    print!("Butler is alive!");

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
