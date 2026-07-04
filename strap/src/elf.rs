use alloc::vec::Vec;

use elf::ElfBytes;
use elf::file::Class;
use elf::abi::{ELFOSABI_SYSV, ET_DYN, ET_EXEC, PT_LOAD, SHT_RELA, R_X86_64_RELATIVE, EM_X86_64, EM_AARCH64, PF_W, PF_R, PF_X, PF_NONE};
use elf::endian::AnyEndian;
use elf::segment::ProgramHeader;

use hal::memory::*;
use core::num::NonZero;
use core::ptr::NonNull;

use hal::paging::*;

pub fn elf_parse_file(page_hierarchy: &mut HalPageHierarchy, frame_allocator: &HalFrameAllocatorWrapper, file: Vec<u8>) -> Option<crate::c_abi::boot_executable_info> {
    log::info!("Parsing elf file");

    let slice = file.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("failed to parse elf file");

    let ehdr = file.ehdr;
    if ehdr.class != Class::ELF64 ||
       ehdr.osabi != ELFOSABI_SYSV ||
       ( if cfg!(target_arch = "x86_64") {
           ehdr.e_machine != EM_X86_64
         } else if cfg!(target_arch = "aarch64") {
             ehdr.e_machine != EM_AARCH64
         } else { true }
       ) || !(ehdr.e_type == ET_EXEC || ehdr.e_type == ET_DYN) {
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
                let total_bytes = (highest_vaddr - lowest_vaddr) as usize;
                let pages = unsafe {
                    frame_allocator.alloc_frames(NonZero::new_unchecked(total_bytes / 0x1000))
                }.expect("could not fetch pages");

                log::info!("Mem range from 0x{:X}", pages.get());
                let pages = unsafe { NonNull::new_unchecked(pages.get() as *const u8 as *mut u8) };
                let pages_u64 = pages.as_ptr() as u64;

                unsafe { core::ptr::write_bytes(pages.as_ptr(), 0, total_bytes); }

                phdrs.iter().for_each(|phdr| {
                    if phdr.p_flags == PF_NONE {
                        page_hierarchy.mapping(pages_u64 + phdr.p_vaddr, pages_u64 + phdr.p_vaddr, HalPageFlags::N, phdr.p_memsz as usize);
                    } else if phdr.p_flags == PF_R {
                        page_hierarchy.mapping(pages_u64 + phdr.p_vaddr, pages_u64 + phdr.p_vaddr, HalPageFlags::R, phdr.p_memsz as usize);
                    } else if phdr.p_flags == PF_R | PF_W {
                        page_hierarchy.mapping(pages_u64 + phdr.p_vaddr, pages_u64 + phdr.p_vaddr, HalPageFlags::RW, phdr.p_memsz as usize);
                    } else if phdr.p_flags == PF_R | PF_X {
                        page_hierarchy.mapping(pages_u64 + phdr.p_vaddr, pages_u64 + phdr.p_vaddr, HalPageFlags::RE, phdr.p_memsz as usize);
                    } else {
                        panic!("invalid combination {}", phdr.p_flags);
                    }

                    unsafe {
                        core::ptr::copy_nonoverlapping(&slice[phdr.p_offset as usize], pages.add(phdr.p_vaddr as usize).as_ptr(), phdr.p_filesz as usize);
                    }
                });

                // relocation time
                let shdrs = file.section_headers().expect("failed to parse section headers");

                // relocation with addends
                for shdr in shdrs.iter().filter(|s| s.sh_type == SHT_RELA) {
                    let relas = file.section_data_as_relas(&shdr).expect("failed to read rela section");

                    for rela in relas {
                        if rela.r_type == R_X86_64_RELATIVE {
                            // patch it
                            let target_ptr = (pages.as_ptr() as u64 + rela.r_offset) as *mut u64;

                            // patched
                            let final_value = (pages.as_ptr() as i64 + rela.r_addend) as u64;

                            unsafe {
                                *target_ptr = final_value;
                            }
                        }
                    }
                }
                
                let stack_bottom = frame_allocator.alloc_frames(NonZero::new(16*4).unwrap()).expect("failed to alloc stack").into();
                page_hierarchy.mapping(stack_bottom, stack_bottom, HalPageFlags::RW, 16* 4 * 4096);
                let stack_top = stack_bottom + 16 * 4 * 4096;
                let kernel_stack_top = stack_top;
                let interrupt_stack_top = kernel_stack_top - 16 * 4096;
                let user_stack_top = interrupt_stack_top - 16 * 4096;

                let entry = pages.as_ptr() as u64 + ehdr.e_entry;
                let info: crate::c_abi::boot_executable_info = crate::c_abi::boot_executable_info {
                    physical_base: pages.as_ptr() as u64,
                    virtual_base: pages.as_ptr() as u64,
                    entry: entry as u64,
                    virtual_space: page_hierarchy.get_root(),
                    kernel_stack_top,
                    interrupt_stack_top,
                    user_stack_top,
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



