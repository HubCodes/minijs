use std::mem::align_of;

pub struct Allocator {
    size: u64,
    ptr: *mut u8,
    last: *mut u8,
}

const THRESHOLD: usize = 4096;

impl Allocator {
    pub fn alloc<T>(&mut self, size: usize, gc_able: bool) -> *mut T {
        unsafe {
            let result = self.last;
            let next_last = next_aligned_ptr(self.last.add(size));
            if self.ptr.add(self.size as usize) > next_last - THRESHOLD && gc_able {
                self.gc();
            }
            result as *mut T
        }
    }

    fn gc(&mut self) {
        todo!();
    }
}

fn next_aligned_ptr(ptr: *mut u8) -> *mut u8 {
    ptr.align_offset(align_of::<u64>()) as *mut u8
}
