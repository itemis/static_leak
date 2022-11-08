//! This crate extends `Mutex`es and `RwLock`s wrapping references with static lifetimes to leak these raw underlying references.
//! This can be useful to safely obtain a static mutable reference without using any unsafe code when interacting with a legacy C project via FFI.
//!
//! ```
//! # use lazy_static::lazy_static;
//! # use std::sync::{RwLock, RwLockWriteGuard};
//! use static_leak::RwLockWriteGuardExtension;
//!
//! lazy_static! {
//!     static ref VAR: RwLock<i8> = RwLock::new(42);
//! }
//!
//! fn bar(_: &'static mut i8) {}
//!
//! fn main() {
//!     bar(RwLockWriteGuard::leak(VAR.write().unwrap()));
//! }
//! ```
//!
//! This crate supports implementations from the following crates, which are activated through the respective feature flags:
//! * std
//! * async-std
//! * spin

#![cfg_attr(not(feature = "std"), no_std)]

/// Improves the support for static references of MutexGuard.
pub trait MutexGuardExtension<T: ?Sized> {
    /// Use this function to leak the raw underlying reference of the MutexGuard.
    fn leak(guard: Self) -> &'static mut T
    where
        Self: Sized;
}

/// Improves the support for static references of RwLockWriteGuard.
pub trait RwLockWriteGuardExtension<T: ?Sized> {
    /// Use this function to leak the raw underlying reference of the WriteGuard.
    fn leak(guard: Self) -> &'static mut T
    where
        Self: Sized;
}

/// Improves the support for static references of RwLockReadGuard.
pub trait RwLockReadGuardExtension<T: ?Sized> {
    /// Use this function to leak the raw underlying reference of the ReadGuard.
    fn leak(guard: Self) -> &'static T
    where
        Self: Sized;
}

#[cfg(feature = "std")]
mod std_impl;

#[cfg(feature = "async-std")]
mod async_std_impl;

#[cfg(feature = "spin")]
mod spin_impl;
