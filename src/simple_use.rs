use crate::allocator_instance::allocator::*;
use std::alloc::{GlobalAlloc, Layout};
use std::error;


/// Allocate memory with give size and return ptr of the address
/// no Hugepage numa use local
pub fn simple_alloicate(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size, 2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc(layout1) };
    ptr
}

/// Deallocate memory with give size and ptr of the address
pub fn simple_dealloicate(ptr: *mut u8, size: usize) {
    let layout1 = Layout::from_size_align(size, 2).unwrap();
    let s = NumaAllocator;
    unsafe {
        s.dealloc(ptr, layout1);
    }
}

/// Allocate memory with give size filled with zero
/// and return ptr of the address
/// no Hugepage numa use local
pub fn simple_alloicate_zero(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size, 2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc_zeroed(layout1) };
    ptr
}

/// Rellocate memory with give size and return ptr of the address
/// no Hugepage numa use local
pub fn simple_realloicate(size: usize) -> *mut u8 {
    let layout1 = Layout::from_size_align(size, 2).unwrap();
    let s = NumaAllocator;
    let ptr = unsafe { s.alloc_zeroed(layout1) };
    ptr
}

/// use function from outside of trait GlobalAlloc to add 'huge_page, node' option
///
/// Use huge_page set it to true. And if have a bind preference set node with 'Some(1)' or 'Some(0)'
/// Currently only support numa with two node. Set node more than '1' will cause panic.
/// If not node is set to 'None', use Local option.
pub fn simple_alloicate_memory_address(
    size: usize,
    huge_page: bool,
    node: Option<u8>,
) -> Result<*mut u8, Box<dyn error::Error>> {
    let layout1 = Layout::from_size_align(size, 2).unwrap();

    let ptr = unsafe {
        let mut res = allocate(layout1, huge_page, node).unwrap(); // NonNull<u8> => *mut u8
        res.as_mut()
    };

    Ok(ptr)
}

// Undone
// pub fn simple_alloicate_zero_memory_address(size: usize) -> *mut u8 {
//     let layout1 = Layout::from_size_align(size ,2).unwrap();
//     let s = NumaAllocator;
//     let ptr = unsafe { s.alloc_zeroed(layout1)};
//     ptr
// }

// Undone
// #[inline(always)]
// unsafe fn reallocate_zeroed(layout: Layout) -> Result<MemoryAddress, AllocErr>{
//     allocator_instance(false).global_alloc_alloc_zeroed_memory_address(layout)
// }

// #[test]
// fn check_numa(){
//     let ptr = simple_alloicate_memory_address(1024, true).unwrap();
//     use std::{thread, time};
//     let time = time::Duration::from_secs(10);
//     println!("ednded");
// }
