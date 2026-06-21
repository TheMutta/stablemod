const KERNEL_MEMORY_MAPPING_OFFSET: u64 = 0xFFFF800000000000;

#[cfg(target_arch = "x86_64")]
use x86_64::{
    structures::paging::{
        FrameAllocator, Mapper, OffsetPageTable, Page, PageTable, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::memory::HalFrameAllocatorWrapper;
use crate::memory::HalFrameAllocatorTrait;

#[cfg(target_arch = "x86_64")]
unsafe impl<T: HalFrameAllocatorTrait> FrameAllocator<Size4KiB> for HalFrameAllocatorWrapper<T> {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.alloc_frame()?;
        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(frame.get()));
        Some(frame)
    }
}

#[cfg(target_arch = "x86_64")]
pub struct HalPageHierarchy<T: HalFrameAllocatorTrait> {
    page_table: OffsetPageTable<'static>,
    frame_allocator: HalFrameAllocatorWrapper<T>,
}

#[cfg(target_arch = "x86_64")]
impl<T: HalFrameAllocatorTrait> HalPageHierarchy<T> {
    pub fn init(table_page: u64, frame_alloc: HalFrameAllocatorWrapper<T>) -> Self {
        unsafe {
            Self {
                page_table: OffsetPageTable::new(&mut *(table_page as *const PageTable as *mut PageTable), VirtAddr::new(KERNEL_MEMORY_MAPPING_OFFSET)),
                frame_allocator: frame_alloc,
            }
        }
    }
    
    pub fn mapping(&mut self, phys_addr: u64, virt_addr: u64) {
        use x86_64::structures::paging::PageTableFlags as Flags;

        let frame = PhysFrame::<Size4KiB>::containing_address(PhysAddr::new(phys_addr));
        let page = Page::containing_address(VirtAddr::new(virt_addr));
        let flags = Flags::PRESENT | Flags::WRITABLE;

        let map_to_result = unsafe {
            self.page_table.map_to(page, frame, flags, &mut self.frame_allocator)
        };
        map_to_result.expect("map_to failed").flush();
    }
}
