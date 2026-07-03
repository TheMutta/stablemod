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

use core::ptr::NonNull;
use core::sync::atomic::AtomicU64;

#[cfg(target_os = "none")]
pub struct HalBareFrameAllocator {
    pointer: UnsafeCell<NonNull<u8>>,
    size: usize,
    page_size: usize,

    memory_base: u64,
    memory_top: u64,

    last_free_page: AtomicU64,
}

#[cfg(target_os = "none")]
impl HalBareFrameAllocator {
    pub const fn new() -> Self {
        Self {
            pointer: UnsafeCell::new(NonNull::dangling()),
            size: 0,
            page_size: 0,

            memory_base: 0,
            memory_top: 0,

            last_free_page: AtomicU64::new(0),
        }
    }

    pub fn init(&mut self, pointer: NonNull<u8>, size: usize, memory_base: u64, memory_top: u64) {
        self.pointer =  UnsafeCell::new(pointer);
        self.size = size;
        self.page_size = 0x1000;
        self.memory_base = memory_base;
        self.memory_top = memory_top;
        self.last_free_page = AtomicU64::default();

        self.set_all_full();
    }

    fn set_all_full(&self) {
        for i in 0..self.size {
            unsafe { *(*self.pointer.get()).as_ptr().add(i) = 0xff; }
        }
    }

    fn calculate_page_offset(&self, page: u64) -> Option<(usize, usize)> {
        if page < self.memory_base || page >= self.memory_top || page % self.page_size as u64 != 0 {
            None
        } else {
            let offset = (page - self.memory_base) / self.page_size as u64;
            let byte_offset = offset / 8;
            let bit_offset = offset % 8;
            Some((byte_offset as usize, bit_offset as usize))
        }
    }
}

pub struct BitmapUtils;
impl BitmapUtils {
    pub fn calculate_mask(offset: usize) -> u8 {
        1 << (offset % 8)
    }

    pub fn is_bit_set(byte: u8, mask: u8) -> bool {
        if (byte & mask) != 0 {
            true
        } else {
            false
        }
    }

    pub fn set_bit(mut byte: u8, mask: u8) -> u8 {
        byte |= mask;

        byte
    }

    pub fn unset_bit(mut byte: u8, mask: u8) -> u8 {
        byte &= !mask;

        byte
    }
}


#[cfg(target_os = "none")]
impl HalFrameAllocatorTrait for HalBareFrameAllocator {
    fn alloc_frame(&self) -> Option<NonZero<u64>> {
        // TODO: Do it allowing for the struct to not be passed as mut
        /*if let Some(page) = self.last_free_page {
        // TODO: We could probably check next + prev to find a new candidate
        self.last_free_page = None;

        let (byte_offset, bit_offset) = self.calculate_page_offset(page)?;
        let mask = Self::calculate_mask(bit_offset);

        let byte = unsafe { *self.pointer.add(byte_offset) };
        let byte = Self::set_bit(byte, mask);
        unsafe { *self.pointer.add(byte_offset) = byte };

        Ok(page)
        } else {
        }*/
        for byte_offset in 0..self.size {
            let byte = unsafe { *(*self.pointer.get()).as_ptr().add(byte_offset) };
            if byte != 0xff {
                for bit_offset in 0..8 {
                    let mask = BitmapUtils::calculate_mask(bit_offset);
                    if !BitmapUtils::is_bit_set(byte, mask) {
                        let byte = BitmapUtils::set_bit(byte, mask);
                        unsafe { *(*self.pointer.get()).as_ptr().add(byte_offset) = byte };
                        let page = (byte_offset * 8 + bit_offset) as u64 * self.page_size as u64 + self.memory_base;

                        return Some(NonZero::new(page).unwrap());
                    }
                }
            }
        }
        None
    }

    fn alloc_frames(&self, count: NonZero<usize>,) -> Option<NonZero<u64>> {
        todo!();
    }

    fn dealloc_frame(&self, frame: NonZero<u64>) {
        // TODO: Add the just freed page to the index
        let page = frame.get();

        let (byte_offset, bit_offset) = self.calculate_page_offset(page).unwrap();
        let mask = BitmapUtils::calculate_mask(bit_offset);

        let byte = unsafe { *(*self.pointer.get()).as_ptr().add(byte_offset) };
        let byte = BitmapUtils::unset_bit(byte, mask);
        unsafe { *(*self.pointer.get()).as_ptr().add(byte_offset) = byte };
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
