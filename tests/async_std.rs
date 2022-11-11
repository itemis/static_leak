#![no_std]
#![cfg(feature = "async-std")]

mod utils;

use async_std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};
use lazy_static::lazy_static;
use static_leak::{MutexGuardExtension, RwLockReadGuardExtension, RwLockWriteGuardExtension};
use utils::{bar, foo, MutexExtension, RwLockExtension};

impl<T: ?Sized> MutexExtension for Mutex<T> {
    fn is_locked(&self) -> bool {
        self.try_lock().is_none()
    }
}

impl<T: ?Sized> RwLockExtension for RwLock<T> {
    fn is_readable(&self) -> bool {
        self.try_read().is_some()
    }
    fn is_writeable(&self) -> bool {
        self.try_write().is_some()
    }
}

lazy_static! {
    static ref VAR1: Mutex<i8> = Mutex::new(42);
}

#[tokio::test]
async fn test_mutexguard_leak() {
    {
        assert!(!VAR1.is_locked());
    }
    {
        let a = VAR1.lock().await;
        foo(MutexGuard::leak(a));
    }
    {
        assert!(VAR1.is_locked());
    }
}

lazy_static! {
    static ref VAR2: RwLock<i8> = RwLock::new(42);
}

#[tokio::test]
async fn test_readguard_leak() {
    {
        assert!(VAR2.is_writeable());
    }
    {
        let a = VAR2.read().await;
        foo(RwLockReadGuard::leak(a));
    }
    {
        assert!(!VAR2.is_writeable());
    }
}

lazy_static! {
    static ref VAR3: RwLock<i8> = RwLock::new(42);
}

#[tokio::test]
async fn test_writeguard_leak() {
    {
        assert!(VAR3.is_readable());
    }
    {
        let a = VAR3.write().await;
        bar(RwLockWriteGuard::leak(a));
    }
    {
        assert!(!VAR3.is_readable());
    }
}
