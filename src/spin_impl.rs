use crate::{MutexGuardExtension, RwLockReadGuardExtension, RwLockWriteGuardExtension};
use core::ops::{Deref, DerefMut};
use spin::{MutexGuard, RwLockReadGuard, RwLockWriteGuard};

impl<T: ?Sized> MutexGuardExtension<T> for MutexGuard<'static, T> {
    fn leak(mut guard: Self) -> &'static mut T
    where
        Self: Sized,
    {
        let r = guard.deref_mut() as *mut _;
        core::mem::forget(guard);
        unsafe { &mut *r }
    }
}

impl<T: ?Sized> RwLockWriteGuardExtension<T> for RwLockWriteGuard<'static, T> {
    fn leak(mut guard: Self) -> &'static mut T
    where
        Self: Sized,
    {
        let r = guard.deref_mut() as *mut _;
        core::mem::forget(guard);
        unsafe { &mut *r }
    }
}

impl<T: ?Sized> RwLockReadGuardExtension<T> for RwLockReadGuard<'static, T> {
    fn leak(guard: Self) -> &'static T
    where
        Self: Sized,
    {
        let r = guard.deref() as *const _;
        core::mem::forget(guard);
        unsafe { &*r }
    }
}
