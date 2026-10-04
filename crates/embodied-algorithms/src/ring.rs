//! Exact-capacity, allocation-free rings. Exclusive Rust borrowing provides synchronization.
use crate::Error;
use core::ops::Index;
pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    len: usize,
}
impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, const N: usize> RingBuffer<T, N> {
    pub const fn new() -> Self {
        Self {
            data: [const { None }; N],
            head: 0,
            len: 0,
        }
    }
    pub const fn len(&self) -> usize {
        self.len
    }
    pub const fn capacity(&self) -> usize {
        N
    }
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub const fn is_full(&self) -> bool {
        self.len == N
    }
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.data[(self.head + self.len) % N] = Some(value);
        self.len += 1;
        Ok(())
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.data[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        v
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        self.data[(self.head + index) % N].as_ref()
    }
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}
impl<T: Copy, const N: usize> RingBuffer<T, N> {
    pub fn write(&mut self, src: &[T]) -> usize {
        let mut n = 0;
        for &v in src {
            if self.push(v).is_err() {
                break;
            }
            n += 1;
        }
        n
    }
    pub fn read(&mut self, dst: &mut [T]) -> usize {
        let mut n = 0;
        for v in dst {
            let Some(x) = self.pop() else {
                break;
            };
            *v = x;
            n += 1;
        }
        n
    }
}
pub struct SlidingWindow<T, const N: usize> {
    data: [T; N],
    oldest: usize,
}
impl<T: Copy, const N: usize> SlidingWindow<T, N> {
    pub fn new(value: T) -> Result<Self, Error> {
        if N == 0 {
            return Err(Error::InvalidParameter);
        }
        Ok(Self {
            data: [value; N],
            oldest: 0,
        })
    }
    pub fn push(&mut self, value: T) {
        self.data[self.oldest] = value;
        self.oldest = (self.oldest + 1) % N;
    }
}
impl<T, const N: usize> Index<usize> for SlidingWindow<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        assert!(index < N);
        &self.data[(self.oldest + index) % N]
    }
}
