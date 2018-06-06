use atomic_ref2::{AtomicOptionRef, IntoOptionArc};
use std::sync::Arc;

/// An array of references in which elements may be updated and retrieved atomically.
///
/// This is a Rust interpretation of [AtomicReferenceArray](https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html) from Java.
pub struct AtomicOptionRefArray<T> {
    buf: Box<[AtomicOptionRef<T>]>,
}

impl<T> AtomicOptionRefArray<T> {
    /// Constructs a new array with the specified length.
    /// All values will be `None`.
    pub fn new(len: usize) -> Self {
        let mut buf = Vec::with_capacity(len);

        for _ in 0..len {
            buf.push(AtomicOptionRef::new());
        }

        Self {
            buf: buf.into_boxed_slice(),
        }
    }

    /// Constructs a new array with the specified length.
    /// Uses the given function to construct each value.
    pub fn new_with<U: IntoOptionArc<T>>(len: usize, f: impl Fn(usize) -> U) -> Self {
        let mut buf = Vec::with_capacity(len);

        for i in 0..len {
            buf.push(AtomicOptionRef::from(f(i)));
        }

        Self {
            buf: buf.into_boxed_slice(),
        }
    }

    /// Returns the number of elements in the array.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if the array has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    /// Loads and returns a reference to an value at the given position or `None`
    /// if the value at the index is not set.
    ///
    /// Panics if `index` is out of bounds.
    pub fn load(&self, index: usize) -> Option<Arc<T>> {
        self.buf[index].load()
    }

    /// Stores the value at the given position.
    ///
    /// Panics if `index` is out of bounds.
    pub fn store(&self, index: usize, value: impl IntoOptionArc<T>) {
        self.buf[index].store(value);
    }

    /// Swaps the value at the given position, returning the previous value.
    ///
    /// Panics if `index` is out of bounds.
    pub fn swap(&self, index: usize, value: impl IntoOptionArc<T>) -> Option<Arc<T>> {
        self.buf[index].swap(value)
    }
}
