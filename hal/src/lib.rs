#![cfg_attr(any(target_os = "none", target_os = "uefi"), no_std)]
#![feature(abi_x86_interrupt)]

pub mod paging;
pub mod memory;
pub mod cpu;
pub mod log;
