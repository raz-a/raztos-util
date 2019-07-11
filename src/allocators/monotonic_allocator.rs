//! # Monotonic Allocator
//!
//! `monotonic_allocator` contains allocator implementations that use a monotonic allocation
//! approach.
//!
//! The heap is an array of `u8` values and the allocator contains a `free_index`. The `free_index`
//! refers to the next byte that is available.
//!
//! The allocator is "monotonic" in the sense that allocation calls will cause the `free_index` to
//! increment, but free calls will not result in a change in the index. This causes memory to be
//! wasted on frees, but gives a realtime guarantee on allocation time.
//!

use core::alloc::{GlobalAlloc, Alloc, Layout, AllocErr};
use core::cell::UnsafeCell;
use core::ptr::NonNull;

/// Defines the structure for the Monotonic Allocator.
/// This type is not thread-safe.
pub struct MonotonicAllocator<'a> (
    UnsafeCell<MonotonicAllocatorInternal<'a>>
);

struct MonotonicAllocatorInternal<'a> {

    /// The heap memory to be given out.
    heap: &'a mut [u8],

    /// Pointer to the next free `u8` in the heap.
    free_index: usize
}

/// Implements the functionality unique to `MonotonicAllocatorInternal`.
impl<'a> MonotonicAllocatorInternal<'a> {

    /// Allocates memory from the MonotonicAllocator.
    ///
    /// # Arguments
    /// layout - provides the memory layout for the requested allocation.
    ///
    /// # Returns
    /// A pointer to the allocated memory if successful.
    /// A null_mut if the allocator doesn't have enough memory or the layout is incompatible with
    ///  the allocator.
    ///
    /// # Unsafe
    /// This function can return a null pointer, a caller must be responsible for handling a null
    /// case.
    unsafe fn alloc_memory(&mut self, layout: Layout) -> *mut u8 {
        let align_mask = layout.align() - 1;
        let aligned_index = (self.free_index + align_mask) & !align_mask;

        if (self.heap.len() - aligned_index) >= layout.size() {
            let out_ptr = self.heap.get_unchecked_mut(aligned_index);
            self.free_index = aligned_index;
            return out_ptr;
        }

        core::ptr::null_mut()
    }
}

/// Implements the functionality unique to `MonotonicAllocator`.
impl<'a> MonotonicAllocator<'a> {

    /// Creates a new MonotonicAllocator struct.
    ///
    /// # Arguments
    /// backing_memory - The caller provided memory to be used for allocation.
    /// Note: The caller is responsible for providing backing memory that is size aligned.
    ///
    /// # Returns
    /// A MonotonicAllocator struct if the provided memory block is valid, otherwise `None`.
    pub fn new(backing_memory: &'a mut [u8]) -> Option<Self> {

        //
        // Verify Alignment
        //

        let memory_ptr_value = backing_memory.as_mut_ptr() as usize;
        let desired_alignment = backing_memory.len().next_power_of_two();
        if memory_ptr_value & (desired_alignment - 1) != 0 {
            return None;
        }

        let allocator = MonotonicAllocator (
            UnsafeCell::new(MonotonicAllocatorInternal {
                heap: backing_memory,
                free_index: 0
            })
        );

        //
        // Zero the backing memory
        //

        let internal = unsafe { &mut *allocator.0.get() };
        for i in internal.heap.iter_mut() {
            *i = 0;
        }

        Some(allocator)
    }

    /// Determines the ammount of free space remaining in the allocator.
    ///
    /// # Returns
    /// Number of free bytes in the allocator.
    pub fn free_space(&self) -> usize {
        let internal = unsafe { &*self.0.get() };
        internal.heap.len() - internal.free_index
    }
}

/// Implements the `GlobalAlloc` trait for `MonotonicAllocator`
///
/// # Unsafe
/// Allocators are inherently unsafe.
unsafe impl<'a> GlobalAlloc for MonotonicAllocator<'a> {

    /// Allocates memory from the MonotonicAllocator.
    ///
    /// # Arguments
    /// layout - provides the memory layout for the requested allocation.
    ///
    /// # Returns
    /// A pointer to the allocated memory if successful.
    /// A null_mut if the allocator doesn't have enough memory or the layout is incompatible with
    ///  the allocator.
    ///
    /// # Unsafe
    /// This function can return a null pointer, a caller must be responsible for handling a null
    /// case.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let internal = &mut *self.0.get();
        internal.alloc_memory(layout)
    }

    /// Frees memory to the MonotonicAllocator.
    ///
    /// # Arguments
    /// _ptr - \[Unused\] The pointer to the memory to free.
    ///
    /// _layout - \[Unused\] The layout of the memory to free.
    ///
    /// # Unsafe
    /// This function does not check for the vailidity of the pointer passed in.
    /// The caller is responsible for providing a pointer to memory provided by this allocator's
    /// `alloc()` function.
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

/// Implements the `Alloc` trait for `MonotonicAllocator`
///
/// # Unsafe
/// Allocators are inherently unsafe.
unsafe impl<'a> Alloc for MonotonicAllocator<'a> {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, AllocErr> {
        let internal = &mut *self.0.get();
        NonNull::new(internal.alloc_memory(layout)).ok_or(AllocErr)
    }

    /// Frees memory to the MonotonicAllocator.
    ///
    /// # Arguments
    /// _ptr - \[Unused\] The pointer to the memory to free.
    ///
    /// _layout - \[Unused\] The layout of the memory to free.
    ///
    /// # Unsafe
    /// This function does not check for the vailidity of the pointer passed in.
    /// The caller is responsible for providing a pointer to memory provided by this allocator's
    /// `alloc()` function.
    unsafe fn dealloc(&mut self, _ptr: NonNull<u8>, _layout: Layout) {}
}
