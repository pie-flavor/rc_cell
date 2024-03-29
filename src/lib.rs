//! Provides a thin wrapper over `Rc<RefCell<T>>` for higher ergonomics, as managing nested types
//! gets annoying.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fmt::{Pointer, Formatter, Result as FmtResult, Display, Error as FmtError};
use std::ops::Deref;

/// The main attraction. Equivalent to `Rc<RefCell<T>>` but with one fewer type. `Rc` methods are
/// reimplemented, and `RefCell` methods are gained by its `Deref` implementation.
///
/// All undocumented methods are equivalent to the `Rc` function of identical name.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct RcCell<T>(Rc<RefCell<T>>);

impl<T> RcCell<T> {
    pub fn new(value: T) -> Self {
        Self(Rc::new(RefCell::new(value)))
    }
    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        Rc::try_unwrap(this.0).map(RefCell::into_inner).map_err(Self)
    }
    pub fn downgrade(&self) -> RcCellWeak<T> {
        RcCellWeak(Rc::downgrade(&self.0))
    }
    pub fn weak_count(&self) -> usize {
        Rc::weak_count(&self.0)
    }
    pub fn strong_count(&self) -> usize {
        Rc::strong_count(&self.0)
    }
    pub fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
    /// Like `swap`, but with another `RcCell` instead of a `RefCell`.
    pub fn swap_with(&self, other: &Self) {
        self.swap(&other.0)
    }
}

impl<T> Deref for RcCell<T> {
    type Target = RefCell<T>;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T> From<T> for RcCell<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> From<Box<T>> for RcCell<T> {
    fn from(t: Box<T>) -> Self {
        Self::new(*t)
    }
}

impl<T> Pointer for RcCell<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:p}", self.0)
    }
}

impl<T> Display for RcCell<T> where T: Display {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.try_borrow().map_err(|_| FmtError)?)
    }
}

/// Equivalent to `Weak<RefCell<T>>` but with one fewer type.
///
/// All undocumented methods are equivalent to the `Rc` function of identical name.
#[derive(Debug, Default, Clone)]
pub struct RcCellWeak<T>(Weak<RefCell<T>>);

impl<T> RcCellWeak<T> {
    pub fn new() -> Self {
        Self(Weak::new())
    }
    pub fn upgrade(&self) -> Option<RcCell<T>> {
        self.0.upgrade().map(RcCell)
    }
    pub fn ptr_eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::RcCell;

    #[test]
    fn basic_test() {
        let cell = RcCell::new("test");
        let cell2 = cell.clone();
        assert_eq!(cell.strong_count(), 2);
        std::mem::drop(cell2);
        assert_eq!(cell.strong_count(), 1);
    }
}
