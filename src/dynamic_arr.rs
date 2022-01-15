use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::{self, NonNull};

#[derive(Debug)]
pub struct DynamicArray<T> {
    len: usize,
    cap: usize,
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> DynamicArray<T> {
    pub fn new() -> Self {
        assert!(
            mem::size_of::<T>() != 0,
            "zero sized types are not supported"
        );
        DynamicArray {
            len: 0,
            cap: 0,
            ptr: NonNull::dangling(),
            _marker: PhantomData,
        }
    }
    pub fn add(&mut self, item: T) {
        // equivalent to push_back
        if self.len == self.cap {
            self.resize();
        }
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }
    pub fn add_at_idx(&mut self, idx: usize, item: T) {
        // usize cannot contain negative numbers so lower bound is implicitly handled
        assert!(idx <= self.len, "index out of bounds");
        if self.len == self.cap {
            self.resize();
        }
        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(idx),
                self.ptr.as_ptr().add(idx + 1),
                self.len - idx,
            );
            ptr::write(self.ptr.as_ptr().add(idx), item);
        }
        self.len += 1
    }
    pub fn delete(&mut self) -> Option<T> {
        // equivalent to pop_back
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }
    pub fn delete_at_idx(&mut self, idx: usize) -> Option<T> {
        if idx >= self.len {
            None
        } else {
            unsafe {
                let rv = ptr::read(self.ptr.as_ptr().add(idx));
                self.len -= 1;
                ptr::copy(
                    self.ptr.as_ptr().add(idx + 1),
                    self.ptr.as_ptr().add(idx),
                    self.len - idx,
                );
                Some(rv)
            }
        }
    }
    pub fn query(&self, i: usize) -> Option<T> {
        if i >= self.len {
            None
        } else {
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(i))) }
        }
    }
    fn resize(&mut self) {
        // equivalent to grow
        let (next_cap, next_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            (self.cap * 2, Layout::array::<T>(self.cap * 2).unwrap())
        };
        assert!(
            next_layout.size() <= isize::MAX as usize,
            "allocation exceeds max size"
        );
        let next_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(next_layout) }
        } else {
            unsafe {
                alloc::realloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                    next_layout.size(),
                )
            }
        };
        // handle alloc failure
        self.ptr = match NonNull::new(next_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(next_layout),
        };
        self.cap = next_cap;
    }
}

impl<T> Drop for DynamicArray<T> {
    fn drop(&mut self) {
        // allocation check
        if self.cap != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                )
            }
        }
    }
}
