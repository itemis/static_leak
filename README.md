# static_leak

[![crates.io](https://img.shields.io/crates/v/static_leak.svg)](https://crates.io/crates/static_leak)
[![Build Status](https://github.com/itemis/static_leak/workflows/Build/badge.svg?branch=main)](https://github.com/itemis/static_leak/actions?query=workflow%3ABuild)
[![docs.rs](https://img.shields.io/docsrs/static_leak)](https://docs.rs/static_leak)
[![codecov](https://codecov.io/gh/itemis/static_leak/branch/main/graph/badge.svg)](https://codecov.io/gh/itemis/static_leak)
![badge](https://img.shields.io/endpoint?url=https%3A%2F%2Fgist.githubusercontent.com%2Fbewee-i%2Ffc6c162ae46b28c4e17f3f358b5ceff2%2Fraw%2Fdoc-coverage.json)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


This crate extends `Mutex`es and `RwLock`s wrapping references with static lifetimes to leak these raw underlying references.
This can be useful to safely obtain a static mutable reference without using any unsafe code when interacting with a legacy C project via FFI.

```rust
lazy_static! {
    static ref VAR: RwLock<i8> = RwLock::new(42);
}

fn bar(_: &'static mut i8) {}

fn main() {
    bar(RwLockWriteGuard::leak(VAR.write().unwrap()));
}
```

This crate supports implementations from the following crates, which are activated through the respective feature flags:
* std
* async-std
* spin
