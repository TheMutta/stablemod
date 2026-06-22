//! # Strap
//! Bootloader

#![allow(warnings)]
#![cfg_attr(target_os = "uefi", no_std)]
#![cfg_attr(target_os = "uefi", no_main)]


/// C abi imports
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

extern crate alloc;

mod config;
mod elf;

#[cfg(target_os = "linux")]
mod posix;

#[cfg(target_os = "uefi")]
mod efi;

#[cfg(any(target_os = "linux"))]
fn main() {
    #[cfg(target_os = "linux")]
    posix::main()
}
