use alloc::vec::Vec;

use elf::ElfBytes;
use elf::file::Class;
use elf::abi::{ELFOSABI_SYSV, ET_DYN, ET_EXEC, PT_LOAD};
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;

use hal::memory::*;
use core::num::NonZero;
use core::ptr::NonNull;

pub fn elf_parse_file<T: HalFrameAllocatorTrait + 'static>(frame_allocator: &HalFrameAllocatorWrapper<T>, file: Vec<u8>) -> Option<crate::c_abi::boot_executable_info> {
    log::info!("Parsing elf file");

    let slice = file.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("failed to parse elf file");

    let ehdr = file.ehdr;
    if ehdr.class != Class::ELF64 ||
       ehdr.osabi != ELFOSABI_SYSV ||
       !(ehdr.e_type == ET_EXEC || ehdr.e_type == ET_DYN) {
        log::info!("Invalid elf file");
        return None;
    }

    let phdrs: Vec<ProgramHeader> = file.segments().expect("failed to parse phdr table").iter().filter(|phdr| phdr.p_type == PT_LOAD).collect();

    let lowest_vaddr = phdrs.iter().map(|phdr| phdr.p_vaddr).min().expect("couldnt calculate lowesdt vaddr");
    let highest_vaddr = phdrs.iter().map(|phdr| phdr.p_vaddr + phdr.p_memsz).max().expect("couldnt calculate highest vaddr");

    let lowest_vaddr = lowest_vaddr - lowest_vaddr % 0x1000;
    let highest_vaddr = if highest_vaddr % 0x1000 != 0 {
        highest_vaddr + (0x1000 - highest_vaddr % 0x1000)
    } else {
        highest_vaddr
    };

    log::info!("File range [0x{:X}-0x{:X}]", lowest_vaddr, highest_vaddr);

    match ehdr.e_type {
        ET_DYN => {
            if lowest_vaddr == 0 {
                let pages = unsafe {
                    frame_allocator.alloc_frames(NonZero::new_unchecked((highest_vaddr - lowest_vaddr) as usize / 0x1000))
                }.expect("could not fetch pages");
                log::info!("Mem range from 0x{:X}", pages.get());
                let pages = unsafe { NonNull::new_unchecked(pages.get() as *const u8 as *mut u8) };

                phdrs.iter().for_each(|phdr| {
                    unsafe {
                        core::ptr::copy_nonoverlapping(&slice[phdr.p_offset as usize], pages.add(phdr.p_vaddr as usize).as_ptr(), phdr.p_filesz as usize);
                    }
                });

                let entry = pages.as_ptr() as u64 + ehdr.e_entry;
                let info: crate::c_abi::boot_executable_info = crate::c_abi::boot_executable_info {
                    physical_base: pages.as_ptr() as u64,
                    virtual_base: pages.as_ptr() as u64,
                    entry: entry as u64,
                    virtual_space: 0,
                    pages: crate::c_abi::boot_executable_info_page_counts {
                        text_pages: 0,
                        rodata_pages: 0,
                        data_pages: 0,
                        null_pages: 0,
                    },
                    crc32: 0,
                    arenas: crate::c_abi::boot_executable_info_capability_arenas {
                        root_resource_capability_arena: 0,
                        root_virtual_capability_arena: 0,
                    }
                };
                log::info!("Executable info: {:#X?}", info);

                Some(info)
            } else {
                None
            }
        },
        ET_EXEC => {
            None
        },
        _ => { None }
    }
}



