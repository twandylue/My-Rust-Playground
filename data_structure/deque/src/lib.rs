use core::slice;
use std::alloc::{alloc, dealloc, realloc};
use std::mem;
use std::ops::{Index, IndexMut};
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
            // TODO:
            1usize << (mem::size_of::<usize>() * 8 - 1)
        } else {
            self.cap
        }
    }

    pub unsafe fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.cast(), self.cap()) }
    }

    pub unsafe fn as_mut_slice(&self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.ptr.cast(), self.cap()) }
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

    fn ptr(&self) -> *mut T {
        self.ring_buf.ptr
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
                    let src = self.ptr();
                    let dst = self.ptr().add(old_cap);
                    ptr::copy_nonoverlapping(src, dst, self.head);
                }
                self.head += old_cap;
            }
        }
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        unsafe { Some(&*self.ptr().add(self.tail)) }
    }

    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        let head = self.wrapping_sub(self.head, 1);
        unsafe { Some(&*self.ptr().add(head)) }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let tail = self.tail;
        self.tail = self.wrapping_add(self.tail, 1);
        unsafe { Some(self.ptr().add(tail).read()) }
    }

    pub fn push_front(&mut self, elem: T) {
        self.try_grow();

        self.tail = self.wrapping_sub(self.tail, 1);
        unsafe {
            self.ptr().add(self.tail).write(elem);
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.head = self.wrapping_sub(self.head, 1);
        unsafe { Some(self.ptr().add(self.head).read()) }
    }

    pub fn push_back(&mut self, elem: T) {
        self.try_grow();
        let head = self.head;
        self.head = self.wrapping_add(self.head, 1);
        unsafe {
            self.ptr().add(head).write(elem);
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            head: self.head,
            tail: self.tail,
            ring_buf: unsafe { self.ring_buf.as_slice() },
        }
    }

    pub fn iter_mut(&self) -> IterMut<T> {
        IterMut {
            head: self.head,
            tail: self.tail,
            ring_buf: unsafe { self.ring_buf.as_mut_slice() },
        }
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_back() {}
    }
}

pub struct Iter<'a, T> {
    head: usize,
    tail: usize,
    ring_buf: &'a [T],
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tail == self.head {
            return None;
        }
        let tail = self.tail;
        self.tail = wrap_index(self.tail.wrapping_add(1), self.ring_buf.len());
        self.ring_buf.get(tail)
    }
}

pub struct IterMut<'a, T> {
    head: usize,
    tail: usize,
    ring_buf: &'a mut [T],
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tail == self.head {
            return None;
        }

        let tail = self.tail;
        self.tail = wrap_index(self.tail.wrapping_add(1), self.ring_buf.len());
        unsafe {
            // NOTE: Should use GAT to resolve lifetime problem
            let ptr = self.ring_buf as *mut [T];
            let slice = &mut *ptr;
            slice.get_mut(tail)
        }
    }
}

impl<'a, T> IntoIterator for &'a Deque<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Deque<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

pub struct IntoIter<T>(Deque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO:
        self.0.pop_front()
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> Index<usize> for Deque<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.len(), "Out of bound");
        let index = self.wrapping_add(self.tail, index);
        unsafe { &*self.ptr().add(index) }
    }
}

impl<T> IndexMut<usize> for Deque<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < self.len(), "Out of bound");
        let index = self.wrapping_add(self.tail, index);
        unsafe { &mut *self.ptr().add(index) }
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
