use super::AtomicOptionRefArray;
use std::sync::Arc;

/// An array of non-optional references in which elements may be updated and retrieved atomically.
pub struct AtomicRefArray<T> {
    buf: AtomicOptionRefArray<T>,
}

impl<T> AtomicRefArray<T> {
    /// Constructs a new array with the specified length.
    /// All values will be initialized to their default.
    pub fn new(len: usize) -> Self
    where
        T: Default,
    {
        Self {
            buf: AtomicOptionRefArray::new_with(len, |_| Some(Arc::new(Default::default()))),
        }
    }

    /// Constructs a new array with the specified length.
    /// Uses the given function to construct each value.
    pub fn new_with(len: usize, f: impl Fn(usize) -> Arc<T>) -> Self {
        Self {
            buf: AtomicOptionRefArray::new_with(len, |i| Some(f(i))),
        }
    }

    /// Returns the number of elements in the array.
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Atomically loads and returns a reference to an value at the given position or `None`
    /// if the value at the index is not set.
    ///
    /// Panics if `index` is out of bounds.
    pub fn load(&self, index: usize) -> Arc<T> {
        self.buf
            .load(index)
            .expect("value in AtomicRefArray is None (should be impossible)")
    }

    /// Atomically stores the value at the given position.
    ///
    /// Panics if `index` is out bounds.
    pub fn store(&self, index: usize, value: impl Into<Arc<T>>) {
        self.buf.store(index, value);
    }
}
