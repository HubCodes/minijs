pub struct Allocator {
    size: u64,
    ptr: *mut u8,
    last: *mut u8,
}

impl Allocator {
    pub fn alloc(&mut self, size: u64) -> *mut u8 {
        unsafe {
            let result = self.last;
            let next_last = self.last.add(size as usize);
            if self.ptr.add(self.size as usize) <= next_last {
                self.gc();
            }
            result
        }
    }

    fn gc(&mut self) {
        todo!();
    }
}
