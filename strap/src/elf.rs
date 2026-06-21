use alloc::vec::Vec;

use elf::ElfBytes;
use elf::file::Class;
use elf::abi::{ELFOSABI_SYSV, ET_DYN, ET_EXEC, PT_LOAD};
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;

use crate::efi;

pub fn elf_parse_file(file: Vec<u8>) {
    log::info!("Parsing elf file");

    let slice = file.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("failed to parse elf file");

    let ehdr = file.ehdr;
    if ehdr.class != Class::ELF64 ||
       ehdr.osabi != ELFOSABI_SYSV ||
       !(ehdr.e_type == ET_EXEC || ehdr.e_type == ET_DYN) {
        log::info!("Invalid elf file");
        return;
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
                let pages = efi::efi_allocate_pages((highest_vaddr - lowest_vaddr) as usize);

                phdrs.iter().for_each(|phdr| {
                    unsafe {
                        core::ptr::copy_nonoverlapping(&slice[phdr.p_offset as usize], pages.add(phdr.p_vaddr as usize).as_ptr(), phdr.p_filesz as usize);
                    }
                });

                /*
                let entry = ehdr.e_entry;
                unsafe {
                    #[cfg(target_arch = "x86_64")]
                    core::arch::asm!("jmp {entry}", entry = in(reg) entry);
                }*/


            } else {

            }
        },
        ET_EXEC => {

        },
        _ => { },
    }
}



