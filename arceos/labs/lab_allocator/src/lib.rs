//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

#[macro_use]
extern crate axlog;

use allocator::{BaseAllocator, ByteAllocator, AllocResult, AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;


pub struct LabByteAllocator {
    start: usize,
    size: usize,
    free: usize,
}

impl LabByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            free: 0,
        }
    }
}

impl BaseAllocator for LabByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        //unimplemented!();
        ax_println!("init_memory: start={:#x}, size={}", start, size);
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        //unimplemented!();
        ax_println!("add_memory: start={:#x}, size={}", start, size);
        //unimplemented!();
        self.start = start;
        self.size = size;
        self.free = 0;
        Ok(())
    }
}

impl ByteAllocator for LabByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        //unimplemented!();
        if self.size < layout.size() {
            return Err(AllocError::NoMemory);
        }

        let alloc_addr = (self.start + self.free) as *mut u8;
        self.free += layout.size();
        Ok(NonNull::new(alloc_addr).unwrap())
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        //unimplemented!();
    }
    fn total_bytes(&self) -> usize {
        //unimplemented!();
        self.size
    }
    fn used_bytes(&self) -> usize {
        //unimplemented!();
        self.free
    }
    fn available_bytes(&self) -> usize {
        //unimplemented!();
        self.size - self.free
    }
}
