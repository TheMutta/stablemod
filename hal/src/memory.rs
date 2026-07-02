#[cfg(target_os = "none")]
pub type HalFrameAllocator = HalBareFrameAllocator;

#[cfg(target_os = "uefi")]
pub type HalFrameAllocator = HalUefiFrameAllocator;

#[cfg(target_os = "linux")]
pub type HalFrameAllocator = HalMemfdFrameAllocator;

pub trait HalFrameAllocatorTrait {
    fn alloc_frame(&self) -> Option<NonZero<u64>>;
    fn alloc_frames(&self, count: NonZero<usize>,) -> Option<NonZero<u64>>;
    fn dealloc_frame(&self, frame: NonZero<u64>);
}

#[derive(Clone)]
pub struct HalFrameAllocatorWrapper {
    inner_allocator: &'static HalFrameAllocator,
}

use core::num::NonZero;

impl HalFrameAllocatorWrapper {
    pub fn new(allocator: &'static HalFrameAllocator) -> Self {
        Self {
            inner_allocator: allocator,
        }
    }

    pub fn alloc_frame(&self) -> Option<NonZero<u64>> {
        self.inner_allocator.alloc_frame()
    }

    pub fn alloc_frames(&self, count: NonZero<usize>) -> Option<NonZero<u64>> {
        self.inner_allocator.alloc_frames(count)
    }
    
    pub fn dealloc_frame(&self, frame: NonZero<u64>) {
        self.inner_allocator.dealloc_frame(frame)
    }
}

#[cfg(target_os = "none")]
pub struct HalBareFrameAllocator {

}

#[cfg(target_os = "none")]
impl HalFrameAllocatorTrait for HalBareFrameAllocator {
    fn alloc_frame(&self) -> Option<NonZero<u64>> {
        todo!();
    }

    fn alloc_frames(&self, count: NonZero<usize>,) -> Option<NonZero<u64>> {
        todo!();
    }

    fn dealloc_frame(&self, frame: NonZero<u64>) {
        todo!();
    }
}

#[cfg(target_os = "uefi")]
pub struct HalUefiFrameAllocator {
}

#[cfg(target_os = "uefi")]
impl HalUefiFrameAllocator {
    pub fn new() -> Self { Self {} }
}

#[cfg(target_os = "uefi")]
impl HalFrameAllocatorTrait for HalUefiFrameAllocator {
    fn alloc_frame(&self) -> Option<NonZero<u64>> {
        use uefi::boot::{allocate_pages, AllocateType, MemoryType};

        let ptr = allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, 1).ok()?;
        Some(unsafe { NonZero::new_unchecked(ptr.as_ptr() as u64) })
    }

    fn alloc_frames(&self, count: NonZero<usize>,) -> Option<NonZero<u64>> {
        use uefi::boot::{allocate_pages, AllocateType, MemoryType};

        let ptr = allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, count.get()).ok()?;
        Some(unsafe { NonZero::new_unchecked(ptr.as_ptr() as u64) })
    }

    fn dealloc_frame(&self, frame: NonZero<u64>) {
        use uefi::boot::free_pages;
        use core::ptr::NonNull;

        unsafe {
            free_pages(NonNull::new_unchecked(frame.get() as *const u8 as *mut u8), 1 );
        }
    }
}


use alloc::vec::Vec;

use core::cell::UnsafeCell;
#[cfg(target_os = "linux")]
pub struct HalMemfdFrameAllocator {
    physical_memory_offset: u64,
    frames: UnsafeCell<Vec<bool>>,
    max_frames: usize,
}

use alloc::vec;

#[cfg(target_os = "linux")]
impl HalMemfdFrameAllocator {
    pub fn new(physical_memory_offset: u64, ram_size_bytes: u64) -> Self {
        const FRAME_SIZE: u64 = 4096;
        let max_frames = (ram_size_bytes / FRAME_SIZE) as usize;
        
        let tracking = vec![false; max_frames];

        Self {
            physical_memory_offset,
            frames: UnsafeCell::new(tracking),
            max_frames,
        }
    }
}


#[cfg(target_os = "linux")]
impl HalFrameAllocatorTrait for HalMemfdFrameAllocator {
    fn alloc_frame(&self) -> Option<NonZero<u64>> {
        let frames = unsafe { &mut *self.frames.get() };
        
        for index in 1..self.max_frames {
            if !frames[index] {
                frames[index] = true;
                let phys_offset = (index as u64) * 4096;
                return Some(unsafe { NonZero::new_unchecked(phys_offset + self.physical_memory_offset) });
            }
        }
        None
    }

    fn alloc_frames(&self, count: NonZero<usize>) -> Option<NonZero<u64>> {
        let frames = unsafe { &mut *self.frames.get() };
        let count_val = count.get();

        if count_val > self.max_frames {
            return None;
        }

        let upper_bound = self.max_frames - count_val;

        for start_index in 1..=upper_bound {
            let end_index = start_index + count_val;

            let is_chunk_free = frames[start_index..end_index].iter().all(|&allocated| !allocated);

            if is_chunk_free {
                frames[start_index..end_index].fill(true);

                let phys_offset = (start_index as u64) * 4096;

                return Some(unsafe { NonZero::new_unchecked(phys_offset + self.physical_memory_offset) });
            }
        }
        None
    }

    fn dealloc_frame(&self, frame: NonZero<u64>) {
        let phys_offset = frame.get() - self.physical_memory_offset;
        let index = (phys_offset / 4096) as usize;
        
        let frames = unsafe { &mut *self.frames.get() };
        if index < self.max_frames {
            frames[index] = false;
        }
    }
}
