//! # Strap
//! Bootloader

#![no_std]
#![no_main]


/// C abi imports
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

extern crate alloc;

mod elf;
mod efi;
