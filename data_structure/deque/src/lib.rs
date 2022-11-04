use std::alloc::{alloc, dealloc, realloc};
use std::mem;
use std::{
    alloc::{handle_alloc_error, Layout},
    ptr,
};

struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
}

impl<T> RawVec<T> {
    pub fn with_capacity(cap: usize) -> Self {
        let layout = Layout::array::<T>(cap).unwrap();
        if layout.size() == 0 {
            let ptr = ptr::NonNull::dangling().as_ptr();
            Self { ptr, cap: 0 }
        } else {
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            Self {
                ptr: ptr.cast(),
                cap,
            }
        }
    }

    fn try_grow(&mut self) {
        if mem::size_of::<T>() == 0 {
            return;
        }
        if self.cap == 0 {
            *self = Self::with_capacity(1);
            return;
        }

        let old_layout = Layout::array::<T>(self.cap).unwrap();
        let new_cap = self.cap << 1;
        let new_size = old_layout.size() * new_cap;
        let ptr = unsafe { realloc(self.ptr.cast(), old_layout, new_size) };
        if ptr.is_null() {
            handle_alloc_error(old_layout);
        }

        self.ptr = ptr.cast();
        self.cap = new_cap;
    }

    pub fn cap(&self) -> usize {
        if mem::size_of::<T>() == 0 {
            1usize << (mem::size_of::<usize>() * 8 - 1)
        } else {
            self.cap
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let layout = Layout::array::<T>(self.cap).unwrap();
        if layout.size() > 0 {
            unsafe { dealloc(self.ptr.cast(), layout) }
        }
    }
}

pub struct Deque<T> {
    tail: usize,
    head: usize,
    ring_buf: RawVec<T>,
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Self {
            tail: 0,
            head: 0,
            ring_buf: RawVec::with_capacity(10),
        }
    }

    fn cap(&self) -> usize {
        self.ring_buf.cap()
    }

    fn wrapping_add(&self, index: usize, addend: usize) -> usize {
        wrap_index(index.wrapping_add(addend), self.cap())
    }

    fn wrapping_sub(&self, index: usize, subend: usize) -> usize {
        wrap_index(index.wrapping_sub(subend), self.cap())
    }

    pub fn is_full(&self) -> bool {
        self.cap() - self.len() == 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.head.wrapping_sub(self.tail) & (self.cap() - 1)
    }

    fn try_grow(&mut self) {
        if self.is_full() {
            let old_cap = self.cap();
            self.ring_buf.try_grow();

            if self.tail > self.head {
                unsafe {
                    let src = self.ring_buf.ptr;
                    let dst = self.ring_buf.ptr.add(old_cap);
                    ptr::copy_nonoverlapping(src, dst, self.head);
                }
                self.head += old_cap;
            }
        }
    }

    pub fn font(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        unsafe { Some(&*self.ring_buf.ptr.add(self.tail)) }
    }

    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let head = self.wrapping_sub(self.head, 1);
        unsafe { Some(&*self.ring_buf.ptr.add(head)) }
    }
}

fn wrap_index(index: usize, size: usize) -> usize {
    debug_assert!(size.is_power_of_two());
    index & (size - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deque_ok() {}
}
