#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod paging;
pub mod memory;
pub mod cpu;
pub mod log;
