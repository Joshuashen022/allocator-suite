use crate::allocator_instance::allocator_instance::*;
use crate::allocators::allocator::Allocator;
use crate::memory_address::MemoryAddress;
use core::ptr::NonNull;
use std::alloc::{AllocError as AllocErr, Allocator as AllocRef, GlobalAlloc, Layout};

#[allow(missing_docs)]
#[derive(Debug, Copy, Clone)]
pub struct NumaAllocator;

#[allow(missing_docs)]
#[global_allocator]
pub static GLOBAL: NumaAllocator = NumaAllocator;

unsafe impl Sync for NumaAllocator {}

/// Allocate memory not using hugepage and with Local mode
unsafe impl GlobalAlloc for NumaAllocator {
    #[inline(always)]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        allocator_instance(false, None).global_alloc_alloc(layout)
    }
    #[inline(always)]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        allocator_instance(false, None).global_alloc_dealloc(ptr, layout)
    }

    #[inline(always)]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        allocator_instance(false, None).global_alloc_alloc_zeroed(layout)
    }

    #[inline(always)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        allocator_instance(false, None).global_alloc_realloc(ptr, layout, new_size)
    }
}

/// Allocate memory not using hugepage and with Local mode
unsafe impl AllocRef for NumaAllocator {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
        let size = layout.size();
        let ptr = unsafe { allocator_instance(false, None).alloc_alloc_zeroed(layout) }?;
        Ok(NonNull::slice_from_raw_parts(ptr, size))
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: MemoryAddress, layout: Layout) {
        allocator_instance(false, None).alloc_dealloc(ptr, layout)
    }
}

/// Numa using Local model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate(
    layout: Layout,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_alloc_memory_address(layout)
}

/// Numa using Local model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate_zeroed_memory_addres(
    ptr: *mut u8,
    layout: Layout,
    lenth: usize,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_realloc_memory_address(ptr, layout, lenth)
}

/// Numa using Prefer(0) model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate_bind0(
    layout: Layout,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_alloc_memory_address(layout)
}
/// Numa using Prefer(0) model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate_bind0_memory_addres(
    ptr: *mut u8,
    layout: Layout,
    lenth: usize,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_realloc_memory_address(ptr, layout, lenth)
}

/// Numa using Prefer(1) model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate_bind1(
    layout: Layout,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_alloc_memory_address(layout)
}

/// Numa using Prefer(1) model
#[allow(dead_code)]
#[inline(always)]
pub unsafe fn allocate_bind1_memory_addres(
    ptr: *mut u8,
    layout: Layout,
    lenth: usize,
    huge_page: bool,
    node: Option<u8>,
) -> Result<MemoryAddress, AllocErr> {
    allocator_instance(huge_page, node).global_alloc_realloc_memory_address(ptr, layout, lenth)
}

// Undone
// #[inline(always)]
// unsafe fn reallocate_zeroed(layout: Layout) -> Result<MemoryAddress, AllocErr>{
//     allocator_instance(false).global_alloc_alloc_zeroed_memory_address(layout)
// }