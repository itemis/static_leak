pub trait MutexExtension {
    fn is_locked(&self) -> bool;
}

pub trait RwLockExtension {
    fn is_readable(&self) -> bool;
    fn is_writeable(&self) -> bool;
}

pub fn foo(reference: &'static i8) {
    assert_eq!(*reference, 42);
}

pub fn bar(reference: &'static mut i8) {
    assert_eq!(*reference, 42);
    *reference = 21;
    assert_eq!(*reference, 21);
}
