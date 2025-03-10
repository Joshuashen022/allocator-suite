#[doc(hidden)]
#[macro_export]
macro_rules! alloc_ref {
    () => {
        #[inline(always)]
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocErr> {
            let size = layout.size();
            let ptr = unsafe { self.alloc_alloc_zeroed(layout) }?;
            Ok(NonNull::slice_from_raw_parts(ptr, size))
        }

        #[inline(always)]
        unsafe fn deallocate(&self, ptr: MemoryAddress, layout: Layout) {
            self.alloc_dealloc(ptr, layout)
        }
    };
}
