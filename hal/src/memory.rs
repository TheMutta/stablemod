pub trait HalFrameAllocatorTrait {
    fn alloc_frame(&self) -> Option<NonZero<u64>>;
    fn dealloc_frame(&self, frame: NonZero<u64>);
}

pub struct HalFrameAllocatorWrapper<T: HalFrameAllocatorTrait> {
    inner_allocator: T,
}

use core::num::NonZero;

impl<T: HalFrameAllocatorTrait> HalFrameAllocatorWrapper<T> {
    pub fn new(allocator: T) -> Self {
        Self {
            inner_allocator: allocator,
        }
    }

    pub fn alloc_frame(&self) -> Option<NonZero<u64>> {
        self.inner_allocator.alloc_frame()
    }
    
    pub fn dealloc_frame(&self, frame: NonZero<u64>) {
        self.inner_allocator.dealloc_frame(frame)
    }
}
