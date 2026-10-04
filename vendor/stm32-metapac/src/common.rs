use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T: Copy, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T: Copy, A: Access> Send for Reg<T, A> {}
unsafe impl<T: Copy, A: Access> Sync for Reg<T, A> {}

impl<T: Copy, A: Access> Reg<T, A> {
    #[allow(clippy::missing_safety_doc)]
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut T) -> Self {
        Self {
            ptr: ptr as _,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut T {
        self.ptr as _
    }
}

impl<T: Copy, A: Read> Reg<T, A> {
    #[inline(always)]
    pub fn read(&self) -> T {
        unsafe { (self.ptr as *mut T).read_volatile() }
    }
}

impl<T: Copy, A: Write> Reg<T, A> {
    #[inline(always)]
    pub fn write_value(&self, val: T) {
        unsafe { (self.ptr as *mut T).write_volatile(val) }
    }
}

impl<T: Default + Copy, A: Write> Reg<T, A> {
    #[inline(always)]
    pub fn write(&self, f: impl FnOnce(&mut T)) {
        let mut val = Default::default();
        f(&mut val);
        self.write_value(val);
    }
}

impl<T: Copy, A: Read + Write> Reg<T, A> {
    #[inline(always)]
    pub fn modify(&self, f: impl FnOnce(&mut T)) {
        let mut val = self.read();
        f(&mut val);
        self.write_value(val);
    }
}

/// Access semantics have not been independently established for this register.
/// Does not implement Read or Write: safe read/modify/default-write are absent.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Raw {}
impl sealed::Access for Raw {}
impl Access for Raw {}
impl<T: Copy> Reg<T, Raw> {
    /// Caller must establish that a read is permitted and its side effects are intended.
    pub unsafe fn read_unchecked(self) -> T { unsafe { core::ptr::read_volatile(self.ptr.cast::<T>()) } }
    /// Caller must establish allowed bits, write semantics and peripheral state.
    pub unsafe fn write_unchecked(self, value: T) { unsafe { core::ptr::write_volatile(self.ptr.cast::<T>(), value) } }
}
