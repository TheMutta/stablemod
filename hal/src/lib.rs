#![cfg_attr(any(target_os = "none", target_os = "uefi"), no_std)]
#![feature(abi_x86_interrupt)]

pub mod paging;
pub mod memory;
pub mod cpu;
pub mod log;

#[cfg(target_os = "none")]
use core::panic::PanicInfo;
#[cfg(target_os = "none")]
#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
