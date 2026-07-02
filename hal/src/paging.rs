const KERNEL_MEMORY_MAPPING_OFFSET: u64 = 0xFFFF800000000000;

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub enum HalPageFlags {
    N,
    R,
    RE,
    RW,
    RWE,
}

use crate::memory::HalFrameAllocatorWrapper;
use crate::memory::HalFrameAllocatorTrait;

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
unsafe impl FrameAllocator<Size4KiB> for HalFrameAllocatorWrapper {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.alloc_frame()?;
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(frame.get()));
        Some(frame)
    }
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
pub struct HalPageHierarchy {
    page_table: OffsetPageTable<'static>,
    frame_allocator: HalFrameAllocatorWrapper,
}

use core::ptr::NonNull;

#[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os= "uefi")))]
pub struct HalPageHierarchy {
    root_page_table: NonNull<u64>,
    frame_allocator: HalFrameAllocatorWrapper,
}

#[cfg(target_os = "linux")]
pub struct HalPageHierarchy {
    physical_mem_fd: i32,
}

impl HalPageHierarchy {
    pub fn init(table_page: u64, frame_alloc: HalFrameAllocatorWrapper) -> Self {
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os= "uefi")))]
        unsafe {
            Self {
                root_page_table: NonNull::new_unchecked(table_page as *mut u64),
                frame_allocator: frame_alloc,
            }
        }
        #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
        unsafe {
            Self {
                page_table: OffsetPageTable::new(&mut *(table_page as *const PageTable as *mut PageTable), VirtAddr::new(0)),
                frame_allocator: frame_alloc,
            }
        }
    
        #[cfg(target_os = "linux")]
        Self {
            physical_mem_fd: table_page as i32,
        }
    }

    pub fn mapping(&mut self, phys_addr: u64, virt_addr: u64, flags: HalPageFlags, length: usize) {
        #[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os= "uefi")))]
        {
            use aarch64_paging::{
                paging::MemoryRegion,
                Mapping
            };
            let region = MemoryRegion::new(virt_addr as usize, virt_addr as usize + length);

        }
        #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
        {
            use x86_64::structures::paging::PageTableFlags as Flags;

            let flags = match flags {
                HalPageFlags::N => Flags::NO_EXECUTE,
                HalPageFlags::R => Flags::PRESENT | Flags::NO_EXECUTE,
                HalPageFlags::RE => Flags::PRESENT,
                HalPageFlags::RW => Flags::PRESENT | Flags::WRITABLE | Flags::NO_EXECUTE,
                HalPageFlags::RWE => Flags::PRESENT | Flags::WRITABLE,
            };

            for off in (0..length).step_by(4096) {
                let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys_addr + off as u64));
                let page = Page::containing_address(VirtAddr::new(virt_addr + off as u64));
                let map_to_result = unsafe {
                    self.page_table.map_to(page, frame, flags, &mut self.frame_allocator)
                };
                map_to_result.expect("map_to failed").flush();
            }
        }
        #[cfg(target_os = "linux")]
        unsafe {
            const PROT_NONE: i32 = 0;
            const PROT_READ: i32 = 0x1;
            const PROT_WRITE: i32 = 0x2;
            const PROT_EXEC: i32 = 0x4;

            const MAP_PRIVATE: i32 = 0x02;
            const MAP_ANONYMOUS: i32 = 0x20;
            const MAP_FIXED: i32 = 0x10;
            let prot = match flags {
                HalPageFlags::N => PROT_NONE,
                HalPageFlags::R => PROT_READ,
                HalPageFlags::RE => PROT_READ | PROT_EXEC,
                HalPageFlags::RW => PROT_READ | PROT_WRITE,
                HalPageFlags::RWE => PROT_READ | PROT_WRITE | PROT_EXEC,
            };

            let flags = MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED;

            let ret = unsafe { syscalls::syscall!(
                syscalls::Sysno::mmap,
                virt_addr as *mut core::ffi::c_void,
                length,
                prot,
                flags,
                self.physical_mem_fd,
                phys_addr
            ).unwrap() };
        }
    }
}
