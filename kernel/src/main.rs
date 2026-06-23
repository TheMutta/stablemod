#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod c_abi {
    include!(concat!(env!("OUT_DIR"), "/abi.rs"));
}

#[cfg(target_os = "none")]
use hal::panic_handler;

#[cfg_attr(target_os = "none", unsafe(no_mangle))]
fn main() {
    loop {}
}
