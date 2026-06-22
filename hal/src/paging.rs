const KERNEL_MEMORY_MAPPING_OFFSET: u64 = 0xFFFF800000000000;

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

pub enum HalPageFlags {
    R,
    RE,
    RW,
    RWE,
}

use crate::memory::HalFrameAllocatorWrapper;
use crate::memory::HalFrameAllocatorTrait;

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
unsafe impl<T: HalFrameAllocatorTrait + 'static> FrameAllocator<Size4KiB> for HalFrameAllocatorWrapper<T> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.alloc_frame()?;
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(frame.get()));
        Some(frame)
    }
}

#[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
pub struct HalPageHierarchy<T: HalFrameAllocatorTrait + 'static> {
    page_table: OffsetPageTable<'static>,
    frame_allocator: HalFrameAllocatorWrapper<T>,
}

#[cfg(all(target_arch = "aarch64", any(target_os = "none", target_os= "uefi")))]
pub struct HalPageHierarchy<T: HalFrameAllocatorTrait + 'static> {
    _phantom: core::marker::PhantomData<T>,
}

#[cfg(target_os = "linux")]
pub struct HalPageHierarchy<T: HalFrameAllocatorTrait + 'static> {
    physical_mem_fd: i32,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: HalFrameAllocatorTrait + 'static> HalPageHierarchy<T> {
    #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
    pub fn init(table_page: u64, frame_alloc: HalFrameAllocatorWrapper<T>) -> Self {
        unsafe {
            Self {
                page_table: OffsetPageTable::new(&mut *(table_page as *const PageTable as *mut PageTable), VirtAddr::new(KERNEL_MEMORY_MAPPING_OFFSET)),
                frame_allocator: frame_alloc,
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn init(physical_mem_fd: i32) -> Self {
        Self {
            physical_mem_fd,
            _phantom: core::marker::PhantomData,
        }
    }
    
    pub fn mapping(&mut self, phys_addr: u64, virt_addr: u64, flags: HalPageFlags) {
        #[cfg(all(target_arch = "x86_64", any(target_os = "none", target_os= "uefi")))]
        {
            use x86_64::structures::paging::PageTableFlags as Flags;

            let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys_addr));
            let page = Page::containing_address(VirtAddr::new(virt_addr));
            let flags = match flags {
                HalPageFlags::R => Flags::PRESENT | Flags::NO_EXECUTE,
                HalPageFlags::RE => Flags::PRESENT,
                HalPageFlags::RW => Flags::PRESENT | Flags::WRITABLE | Flags::NO_EXECUTE,
                HalPageFlags::RWE => Flags::PRESENT | Flags::WRITABLE,
            };

            let map_to_result = unsafe {
                self.page_table.map_to(page, frame, flags, &mut self.frame_allocator)
            };
            map_to_result.expect("map_to failed").flush();
        }
        #[cfg(target_os = "linux")]
        unsafe {
            let prot = match flags {
                HalPageFlags::R => libc::PROT_READ,
                HalPageFlags::RE => libc::PROT_READ | libc::PROT_EXEC,
                HalPageFlags::RW => libc::PROT_READ | libc::PROT_WRITE,
                HalPageFlags::RWE => libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            };

            let flags = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_FIXED;

            let ret = libc::mmap(
                virt_addr as *mut libc::c_void,
                4096, // Size4KiB
                prot,
                flags,
                self.physical_mem_fd,
                phys_addr as libc::off_t,
            );

            assert!(ret != libc::MAP_FAILED, "Failed to sync mapping to Linux host MMU");
        }
    }
}
