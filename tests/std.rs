#![cfg(feature = "std")]

mod utils;

use utils::{MutexExtension, RwLockExtension, foo, bar};
use lazy_static::lazy_static;
use static_leak::{MutexGuardExtension, RwLockReadGuardExtension, RwLockWriteGuardExtension};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

impl<T: ?Sized> MutexExtension for Mutex<T> {
    fn is_locked(&self) -> bool {
        self.try_lock().is_err()
    }
}

impl<T: ?Sized> RwLockExtension for RwLock<T> {
    fn is_readable(&self) -> bool {
        self.try_read().is_ok()
    }
    fn is_writeable(&self) -> bool {
        self.try_write().is_ok()
    }
}

lazy_static! {
    static ref VAR1: Mutex<i8> = Mutex::new(42);
}

#[test]
fn test_mutexguard_leak() {
    {
        assert!(!VAR1.is_locked());
    }
    {
        let a = VAR1.lock();
        assert!(a.is_ok());
        foo(MutexGuard::leak(a.unwrap()));
    }
    {
        assert!(VAR1.is_locked());
    }
}

lazy_static! {
    static ref VAR2: RwLock<i8> = RwLock::new(42);
}

#[test]
fn test_readguard_leak() {
    {
        assert!(VAR2.is_writeable());
    }
    {
        let a = VAR2.read();
        assert!(a.is_ok());
        foo(RwLockReadGuard::leak(a.unwrap()));
    }
    {
        assert!(!VAR2.is_writeable());
    }
}

lazy_static! {
    static ref VAR3: RwLock<i8> = RwLock::new(42);
}

#[test]
fn test_writeguard_leak() {
    {
        assert!(VAR3.is_readable());
    }
    {
        let a = VAR3.write();
        assert!(a.is_ok());
        bar(RwLockWriteGuard::leak(a.unwrap()));
    }
    {
        assert!(!VAR3.is_readable());
    }
}
