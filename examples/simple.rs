
use lazy_static::*;
use core::ptr::NonNull;
use std::alloc::{GlobalAlloc, Allocator as AllocRef, Layout, AllocError as AllocErr};

use crate::adaptors::prelude::*;
use crate::allocators::allocator::Allocator;
use crate::memory_address::MemoryAddress;
use crate::allocators::memory_map_allocator::MemoryMapAllocator;

#[allow(missing_docs)]
fn allocator_instance() -> &'static AllocatorAdaptor<'static, MemoryMapAllocator> {
    use crate::memory_sources::mmap::prelude::*;
    lazy_static! {
                static ref MMAP_ALLOC: MemoryMapAllocator = {
                    #[cfg(any(target_os = "android", target_os = "linux"))] {
                        use crate::memory_sources::mmap::numa::numa_settings::NumaSettings;
                        use crate::memory_sources::mmap::numa::numa_allocation_policy::NumaAllocationPolicy;
                        let numa_settings = NumaSettings::new(
                            NumaAllocationPolicy::Local,
                            true,
                        );

                        let mmap = MemoryMapSource::new(
                            true,
                            false,
                            false,
                            false,
                            HugePageSize::default(),
                            Some(numa_settings)
                        );
                        MemoryMapAllocator(mmap)
                    }

                    #[cfg(not(any(target_os = "android", target_os = "linux")))] {
                        let mmap = MemoryMapSource::default();
                        MemoryMapAllocator(mmap)
                    }
                };


                static ref MMAP_ADAPTER: AllocatorAdaptor<'static, MemoryMapAllocator> = {
                    MMAP_ALLOC.adapt()
                };
            }

    &*MMAP_ADAPTER
}

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub struct NumaAllocator;

#[allow(missing_docs)]
#[global_allocator]
pub static GLOBAL: NumaAllocator = NumaAllocator;

unsafe impl Sync for NumaAllocator {}

unsafe impl GlobalAlloc for NumaAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8
    {
        allocator_instance().global_alloc_alloc(layout)
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout)
    {
        allocator_instance().global_alloc_dealloc(ptr, layout)
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8
    {
        allocator_instance().global_alloc_alloc_zeroed(layout)
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
    {
        allocator_instance().global_alloc_realloc(ptr, layout, new_size)
    }
}

unsafe impl AllocRef for NumaAllocator {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr>
    {
        let size = layout.size();
        let ptr = unsafe { allocator_instance().alloc_alloc_zeroed(layout) }?;
        Ok(NonNull::slice_from_raw_parts(ptr, size))
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: MemoryAddress, layout: Layout)
    {
        allocator_instance().alloc_dealloc(ptr, layout)
    }
}

pub fn simple_alloicate(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size ,2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc(layout1)};
    ptr
}

pub fn simple_dealloicate(ptr: *mut u8, size: usize) {
    let layout1 = Layout::from_size_align(size ,2).unwrap();
    let s = NumaAllocator;
    unsafe {s.dealloc(ptr ,layout1);}
}

pub fn simple_alloicate_zero(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size ,2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc_zeroed(layout1)};
    ptr
}

pub fn simple_realloicate(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size ,2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc_zeroed(layout1)};
    ptr
}

fn main() {
    // Allocated by Global switchable allocator's global allocator.
    let length  = 128;
    let ptr = simple_alloicate(length);
    simple_dealloicate(ptr, length);
}
