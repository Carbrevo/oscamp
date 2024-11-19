//! Allocator algorithm in lab.

#![no_std]
#![allow(unused_variables)]

#[macro_use]
extern crate axlog;

use allocator::{BaseAllocator, ByteAllocator, AllocResult, AllocError};
use core::ptr::NonNull;
use core::alloc::Layout;
//use core::alloc::vec::Vec;

struct MemBlk {
    start: usize,
    size: usize,
}

impl MemBlk {
    fn new(start: usize, size: usize) -> Self {
        Self {
            start,
            size,
        }
    }
}

struct MemBlkPool<'a> {
    memblk: MemBlk,
    next: Option<&'a MemBlkPool<'a>>,
}

impl<'a> MemBlkPool<'a> {
    fn new(start: usize, size: usize) -> Self {
        Self {
            memblk: MemBlk::new(start, size),
            next: None,
        }
    }

    fn add_blk(&'a mut self, start: usize, size: usize) {
        assert!(self.next.is_none());
        self.next = Some(&MemBlkPool::new(start, size));
    }

    fn size(&self) -> usize {
        let total = 0;
        let next = self;
        loop {
            total += next.memblk.size;
            if next.next.is_none() {
                break;
            }
            next = next.next.unwrap();
        }
        total
    }
}

struct MemBlkUsed<'a> {
    memblk: MemBlk,
    next: Option<&'a MemBlkUsed<'a>>,
}

impl<'a> MemBlkUsed<'a> {
    fn new(blk: &MemBlk) -> Self {
        Self {
            memblk: MemBlk::new(blk.start, blk.size),
            next: None,
        }
    }

    fn add_blk(&mut self, blk: &MemBlk) {
        assert!(self.next.is_none());
        self.next = Some(&MemBlkUsed::new(blk));
    }

    fn size(&self) -> usize {
        let total = 0;
        let next = self;
        loop {
            total += next.memblk.size;
            if next.next.is_none() {
                break;
            }
            next = next.next.unwrap();
        }
        total
    }
}
pub struct LabByteAllocator<'a> {
    free: Option<&'a MemBlkPool<'a>>,
    used: Option<&'a MemBlkUsed<'a>>,
}

impl<'a> LabByteAllocator<'a> {
    pub const fn new() -> Self {
        Self {
            free: None,
            used: None,
       }
    }
}

impl<'a> BaseAllocator for LabByteAllocator<'a> {
    fn init(&mut self, start: usize, size: usize) {
        //unimplemented!();
        ax_println!("init_memory: start={:#x}, size={}", start, size);
        self.free = Some(&MemBlkPool::new(start, size));
    }
    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        //unimplemented!();
        ax_println!("add_memory: start={:#x}, size={}", start, size);
        if self.free.is_none() {
            Ok(self.free = Some(&MemBlkPool::new(start, size)))
        } else {
            Ok(self.free.unwrap().add_blk(start, size))
        }
    }
}

impl<'a> ByteAllocator for LabByteAllocator<'a> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        //unimplemented!();
        if self.free.is_none() {
            return Err(AllocError::NoMemory);
        }
        let found = self.free.unwrap();
        let prev: Option<&'static MemBlkPool> = None;
        loop {
            if found.memblk.size >= layout.size() {
                if prev.is_none() {
                    self.free = found.next;
                } else {
                    prev.unwrap().next = found.next;
                }
                found.next = None;
                
                if self.used.is_none() {
                    self.used = Some(&MemBlkUsed::new(&found.memblk));
                } else {
                    self.used.unwrap().add_blk(&found.memblk);
                }

                let alloc_addr = (found.memblk.start) as *mut u8;
                return Ok(NonNull::new(alloc_addr).unwrap())
            } else {
                if found.next.is_none() {
                    break;
                }

                prev = Some(found);
                found = found.next.unwrap();
            }
        }

        ax_println!("memory total={:#x}, used={:#x}, fragment={:#x}",
                    self.total_bytes(), self.used.unwrap().size(), self.free.unwrap().size());

        return Err(AllocError::NoMemory);
    }
    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        //unimplemented!();

    }
    fn total_bytes(&self) -> usize {
        //unimplemented!();
        let total: usize = 0;
        if let Some(free_pool) = self.free {
            total += free_pool.size();
        }
        if let Some(used_pool) = self.used {
            total += used_pool.size();
        }
        total
    }
    
    fn used_bytes(&self) -> usize {
        //unimplemented!();
        if let Some(used_pool) = self.used {
            used_pool.size()
        } else {
            0usize
        }
    }
    fn available_bytes(&self) -> usize {
        //unimplemented!();
        if let Some(free_pool) = self.free {
            free_pool.size()
        } else {
            0usize
        }
    }
}
