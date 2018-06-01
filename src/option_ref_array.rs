use std::mem;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Arc;

/// An array of references in which elements may be updated and retrieved atomically.
///
/// This is a Rust interpretation of [AtomicReferenceArray](https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html) from Java.
pub struct AtomicOptionRefArray<T> {
    buf: Box<[AtomicPtr<T>]>,
}

impl<T> AtomicOptionRefArray<T> {
    /// Constructs a new array with the specified length.
    /// All values will be `None`.
    pub fn new(len: usize) -> Self {
        let mut buf = Vec::with_capacity(len);

        for _ in 0..len {
            buf.push(AtomicPtr::new(null_mut()));
        }

        Self {
            buf: buf.into_boxed_slice(),
        }
    }

    /// Constructs a new array with the specified length.
    /// Uses the given function to construct each value.
    pub fn new_with(len: usize, f: impl Fn(usize) -> Option<Arc<T>>) -> Self {
        let mut buf = Vec::with_capacity(len);

        for i in 0..len {
            let value = f(i).map_or_else(null_mut, |value| Arc::into_raw(value) as *mut _);

            buf.push(AtomicPtr::new(value));
        }

        Self {
            buf: buf.into_boxed_slice(),
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
    pub fn load(&self, index: usize) -> Option<Arc<T>> {
        let value = self.buf[index].load(Ordering::SeqCst);
        if value.is_null() {
            // Return `None` if null is stored in the AtomicPtr
            None
        } else {
            // Otherwise, reconstruct the stored Arc
            let value = unsafe { Arc::from_raw(value) };

            // And create a new reference to return
            let value_ = Arc::clone(&value);

            // Forget the reconstructed Arc (as its still in the array as a raw ptr)
            mem::forget(value);

            // And return our new reference
            Some(value_)
        }
    }

    /// Atomically stores the value at the given position.
    ///
    /// Panics if `index` is out of bounds.
    pub fn store(&self, index: usize, value: impl Into<Arc<T>>) {
        let value = Arc::into_raw(value.into()) as *mut _;

        self.buf[index].store(value, Ordering::SeqCst);
    }
}

impl<T> Drop for AtomicOptionRefArray<T> {
    fn drop(&mut self) {
        for value in self.buf.iter() {
            let value = value.swap(null_mut(), Ordering::SeqCst);
            if !value.is_null() {
                unsafe {
                    // Reconstruct the Arc from the raw ptr which will trigger our destructor
                    // if there is one
                    let _ = Arc::from_raw(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AtomicOptionRefArray;

    #[test]
    fn test_store_load() {
        let m = AtomicOptionRefArray::<String>::new(5);

        // Store several values
        for i in 0..5 {
            m.store(i, format!("{}", i));
        }

        // Load the values and assert the values
        for i in 0..5 {
            assert_eq!(m.load(i).unwrap().as_ref(), &format!("{}", i));
        }
    }

    #[test]
    fn test_overwrite() {
        let m = AtomicOptionRefArray::<String>::new(5);

        // Store at #0
        m.store(0, String::from("Hello World"));

        // Take a reference to #0
        let m0 = m.load(0);

        // Store at #0 (again)
        m.store(0, String::from("Goodbye World"));

        // Compare value of stored #0
        assert_eq!(m0.unwrap().as_ref(), "Hello World");

        // Compare value of new #0
        assert_eq!(m.load(0).unwrap().as_ref(), "Goodbye World");
    }
}
