use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

const MAX_SUPPORTED_ALIGN: usize = 64;
#[repr(C, align(64))] // 64 == MAX_SUPPORTED_ALIGN
struct LibCAllocator {}

#[global_allocator]
static ALLOCATOR: LibCAllocator = LibCAllocator {};

unsafe impl Sync for LibCAllocator {}

unsafe impl GlobalAlloc for LibCAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask = align - 1;

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        // Round up size to nearest multiple of alignment.
        let size = (size + align_mask) & !align_mask;

        malloc(size)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr);
    }
}
