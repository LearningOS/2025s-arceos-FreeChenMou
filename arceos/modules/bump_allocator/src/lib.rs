#![no_std]

use allocator::{AllocError, AllocResult, BaseAllocator, ByteAllocator, PageAllocator};
use core::alloc::{Layout};
use core::cmp;
use core::ptr::null_mut;
use core::ptr::NonNull;
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE: usize> {
    start: usize,
    end: usize,
    b_pos: usize,
    p_pos: usize,
    byte_alloc_count: usize,
    byte_alloc_total: usize,
    page_alloc_total: usize,
    page_alloc_count: usize,
}

impl<const PAGE_SIZE: usize> EarlyAllocator<PAGE_SIZE> {
    pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            b_pos: 0,
            p_pos: 0,
            byte_alloc_count: 0,
            byte_alloc_total: 0,
            page_alloc_total: 0,
            page_alloc_count: 0,
        }
    }
}

impl<const PAGE_SIZE: usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.end = start + size;
        self.b_pos = start;
        self.p_pos = self.end;
        self.byte_alloc_count = 0;
        self.byte_alloc_total = 0;
        self.page_alloc_total = 0;
        self.page_alloc_count = 0;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        !todo!();
    }
}

impl<const PAGE_SIZE: usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();
        let aligned = (self.b_pos + align - 1) & !(align - 1);
        let new_b_pos = aligned + size;

        if new_b_pos > self.p_pos {
            return Err(AllocError::NoMemory);
        }

        self.b_pos = new_b_pos;
        self.byte_alloc_count += 1;
        self.byte_alloc_total += size;
        Ok(NonNull::new(aligned as *mut u8).unwrap())
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, layout: Layout) {
        self.byte_alloc_count -= 1;
        self.byte_alloc_total -= layout.size();
        if self.byte_alloc_count == 0 {
            self.b_pos = self.start;
            self.byte_alloc_total = 0;
        }
    }

    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
        self.byte_alloc_total
    }

    fn available_bytes(&self) -> usize {
        self.p_pos - self.b_pos
    }
}

impl<const PAGE_SIZE: usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow: usize) -> AllocResult<usize> {
        let align = 1 << align_pow;
        let size = num_pages * PAGE_SIZE;
        let aligned = (self.p_pos - size) & !(align - 1);

        if aligned < self.b_pos {
            return Err(AllocError::NoMemory);
        }

        self.p_pos = aligned;
        self.page_alloc_total += size;
        self.page_alloc_count += num_pages;
        Ok(aligned)
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        !todo!();
    }

    fn total_pages(&self) -> usize {
        (self.end - self.start) / PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
        self.page_alloc_total / PAGE_SIZE
    }

    fn available_pages(&self) -> usize {
        (self.p_pos - self.b_pos) / PAGE_SIZE
    }
}