#![cfg(feature = "spin")]
#![no_std]

mod utils;

use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use static_leak::{MutexGuardExtension, RwLockReadGuardExtension, RwLockWriteGuardExtension};
use utils::{bar, foo, RwLockExtension};

impl<T: ?Sized> RwLockExtension for RwLock<T> {
    fn is_readable(&self) -> bool {
        self.writer_count() == 0
    }
    fn is_writeable(&self) -> bool {
        self.writer_count() == 0 && self.reader_count() == 0
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
        foo(<MutexGuard<'_, _> as MutexGuardExtension<_>>::leak(a));
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
        foo(<RwLockReadGuard<'_, _> as RwLockReadGuardExtension<_>>::leak(a));
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
        bar(<RwLockWriteGuard<'_, _> as RwLockWriteGuardExtension<_>>::leak(a));
    }
    {
        assert!(!VAR3.is_readable());
    }
}
