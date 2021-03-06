//! `RefCell` accessible only from the main thread.

use std::cell::{Ref, RefCell, RefMut};

use crate::utils::*;

/// `RefCell` accessible only from the main thread.
pub struct MainThreadRefCell<T>(RefCell<T>);

// Safety: all methods are guarded with MainThreadMarker.
unsafe impl<T> Send for MainThreadRefCell<T> {}
unsafe impl<T> Sync for MainThreadRefCell<T> {}

impl<T> MainThreadRefCell<T> {
    /// Creates a new `MainThreadRefCell` containing the given value.
    pub const fn new(value: T) -> Self {
        Self(RefCell::new(value))
    }

    /// Immutably borrows the wrapped value.
    #[allow(unused)]
    pub fn borrow(&self, _marker: MainThreadMarker) -> Ref<T> {
        self.0.borrow()
    }

    /// Mutably borrows the wrapped value.
    pub fn borrow_mut(&self, _marker: MainThreadMarker) -> RefMut<T> {
        self.0.borrow_mut()
    }
}
