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
static mut ROOT_RESOURCE_CAPABILITY_ARENA: *mut ResourceCapabilityArena = null_mut();
static mut ROOT_VIRTUAL_CAPABILITY_ARENA: *mut ResourceCapabilityArena = null_mut();

extern "C" fn sys_debug(s: *const u8, count: usize) -> usize {
    let slice = unsafe { core::slice::from_raw_parts(s, count) };
    log::info!("userspace says: {}", str::from_utf8(slice).unwrap());

    0
}


use kernel::*;

extern "C" fn sys_resource_arena_create(
    arena_handle: *const CapabilityHandle,
    cap_handle: *const CapabilityHandle,
    arena_cap_handle: *mut *const CapabilityHandle) -> usize {

    if arena_handle.is_null() || cap_handle.is_null() || arena_cap_handle.is_null() {
return 1;
    }
    
    let _arena_handle = unsafe { &*arena_handle };
    let _cap_handle = unsafe { &*cap_handle };
    let _arena_cap_hanle = unsafe { &mut*arena_cap_handle };

    todo!();
}

extern "C" fn sys_resource_arena_destroy(
    arena_handle: *const CapabilityHandle) -> usize {

    if arena_handle.is_null() {
        return 1;
    }

    let _arena_handle = unsafe { &*arena_handle };

    todo!();
}

extern "C" fn sys_virtual_arena_create(
    _arena_handle: *const CapabilityHandle,
    _cap_handle: *const CapabilityHandle,
    _arena_cap_handle: *mut *const CapabilityHandle) {

    todo!();
}

extern "C" fn sys_virtual_arena_destroy(
    _arena_handle: *const CapabilityHandle) {

    todo!();
}

extern "C" fn sys_cap_derive(
    _arena_handle: *const CapabilityHandle,
    _cap_handle: *const CapabilityHandle,
    _perms: u64,
    _dest_arena_handle: *const CapabilityHandle,
    _dest_cap_handle: *const CapabilityHandle) {

    todo!();
}

extern "C" fn sys_cap_split(
    _arena_handle: *const CapabilityHandle,
    _cap_handle: *const CapabilityHandle,
    _dest_arena_handle: *const CapabilityHandle,
    _off: u64,
    _dest_cap_handle_left: *mut *const CapabilityHandle,
    _dest_cap_handle_right: *mut *const CapabilityHandle) {


    todo!();
}

extern "C" fn sys_cap_revoke(
    _arena_handle: *const CapabilityHandle,
    _cap_handle: *const CapabilityHandle) {

    todo!();
}

extern "C" fn sys_cap_write(
    arena_handle: *const CapabilityHandle,
    cap_handle: *const CapabilityHandle,
    off: u64,
    buffer: *const u8,
    len: u64) -> usize {

    if arena_handle.is_null() || cap_handle.is_null() || buffer.is_null() {
        return 1;
    }

    let arena_handle = unsafe { &*arena_handle };
    let cap_handle = unsafe { &*cap_handle };

    if arena_handle.arena_id != cap_handle.arena_id {
        return 1;
    }
    
    use kernel::tree::find_arena;
    let arena = unsafe {
        find_arena(ROOT_RESOURCE_CAPABILITY_ARENA, arena_handle.arena_id)
    };

    if arena.is_null() {
        return 1;
    }

    let arena = unsafe { &*arena };

    for slot in arena.get_slots() {
        if slot.genid == cap_handle.generation_id {
            if (slot.permissions & crate::c_abi::CAPABILITY_WRITE) != 0 {
                if off + len > slot.size {
                    return 1;
                }

                unsafe {
                    core::ptr::copy_nonoverlapping(buffer, (slot.resource + off) as *mut u8, len as usize);
                }
            }

            return 0;
        }
    }

    return 1;
}

extern "C" fn sys_cap_read(
    arena_handle: *const CapabilityHandle,
    cap_handle: *const CapabilityHandle,
    off: u64,
    buffer: *mut u8,
    len: u64) -> usize {

    if arena_handle.is_null() || cap_handle.is_null() || buffer.is_null() {
        return 1;
    }

    let arena_handle = unsafe { &*arena_handle };
    let cap_handle = unsafe { &*cap_handle };

    if arena_handle.arena_id != cap_handle.arena_id {
        return 1;
    }
    
    use kernel::tree::find_arena;
    let arena = unsafe {
        find_arena(ROOT_RESOURCE_CAPABILITY_ARENA, arena_handle.arena_id)
    };

    if arena.is_null() {
        return 1;
    }

    let arena = unsafe { &*arena };

    for slot in arena.get_slots() {
        if slot.genid == cap_handle.generation_id {
            if (slot.permissions & crate::c_abi::CAPABILITY_READ) != 0 {
                if off + len > slot.size {
                    return 1;
                }

                unsafe {
                    core::ptr::copy_nonoverlapping((slot.resource + off) as *const u8, buffer, len as usize);
                }
            }

            return 0;
        }
    }

    return 1;
}

extern "C" fn sys_cap_map(
    _arena_handle: *const CapabilityHandle,
    _cap_handle: *const CapabilityHandle,
    _virt_arena_handle: *const CapabilityHandle) {

    todo!();
}

extern "C" fn sys_cap_unmap(
    _virt_arena_handle: *const CapabilityHandle,
    _virt_cap_handle: *const CapabilityHandle) {

    todo!();
}

batch_syscalls!(
    sys_0_handler => sys_debug,
    sys_1_handler => sys_resource_arena_create,
    sys_2_handler => sys_resource_arena_destroy,
    sys_3_handler => sys_virtual_arena_create,
    sys_4_handler => sys_virtual_arena_destroy,
    sys_5_handler => sys_cap_derive,
    sys_6_handler => sys_cap_split,
    sys_7_handler => sys_cap_revoke,
    sys_8_handler => sys_cap_write,
    sys_9_handler => sys_cap_read,
    sys_10_handler => sys_cap_map,
    sys_11_handler => sys_cap_unmap,
);


#[unsafe(no_mangle)]
extern "C" fn _start(bootloader_data: *const crate::c_abi::boot_loader_data) -> ! { 
    HalLogger::init();

    log::info!("hello, world!");
    log::info!("this is the kernel speaking!");

    let bootloader_data = unsafe { &(*bootloader_data) };
    if bootloader_data.signature != crate::c_abi::BOOTLOADER_SIGNATURE {
        panic!("Invalid bootloader data!");
    }


    log::info!("bootloader data: {:#?}", bootloader_data);


    unsafe {
        ROOT_RESOURCE_CAPABILITY_ARENA = bootloader_data.kernel_executable.arenas.root_resource_capability_arena as *mut ResourceCapabilityArena;
    }

    let frame_allocator = unsafe {
        //FRAME_ALLOCATOR.init();
        HalFrameAllocatorWrapper::new(&FRAME_ALLOCATOR)
    };

    unsafe {
        BOOT_PROCESSOR.init();
    }
    
    log::info!("cpu initialised");

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
