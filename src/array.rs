use std::sync::atomic::*;

macro_rules! impl_array {
    ($name:ident, $type_name:expr, $type:ty, $atomic_type:ident) => {
        #[doc = "A `"]
        #[doc = $type_name]
        #[doc = "` array in which elements may be updated atomically."]
        pub struct $name {
            buf: Box<[$atomic_type]>
        }

        impl $name {
            /// Constructs a new array with the specified length.
            /// All values will be initialized to their default.
            pub fn new(len: usize) -> Self {
                Self::new_with(len, |_| Default::default())
            }

            /// Constructs a new array with the specified length.
            /// Uses the given function to construct each value.
            pub fn new_with(len: usize, f: impl Fn(usize) -> $type) -> Self {
                let mut buf = Vec::with_capacity(len);

                for i in 0..len {
                    buf.push($atomic_type::new(f(i)));
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

            /// Loads and returns the value at the given position.
            ///
            /// Panics if `index` is out of bounds.
            pub fn load(&self, index: usize) -> $type {
                self.buf[index].load(Ordering::SeqCst)
            }

            /// Stores the value at the given position.
            ///
            /// Panics if `index` is out bounds.
            pub fn store(&self, index: usize, value: $type) {
                self.buf[index].store(value, Ordering::SeqCst)
            }

            /// Swaps the value at the given position, returning the previous value.
            ///
            /// Panics if `index` is out of bounds.
            pub fn swap(&self, index: usize, value: $type) -> $type {
                self.buf[index].swap(value, Ordering::SeqCst)
            }
        }
    };
}

impl_array!(AtomicBoolArray, "bool", bool, AtomicBool);

#[cfg(feature = "integer_atomics")]
mod integer {
    impl_array!(AtomicI8Array, "i8", i8, AtomicI8);
    impl_array!(AtomicI16Array, "i16", i16, AtomicI16);
    impl_array!(AtomicI32Array, "i32", i32, AtomicI32);
    impl_array!(AtomicI64Array, "i64", i64, AtomicI64);

    impl_array!(AtomicU8Array, "u8", u8, AtomicU8);
    impl_array!(AtomicU16Array, "u16", u16, AtomicU16);
    impl_array!(AtomicU32Array, "u32", u32, AtomicU32);
    impl_array!(AtomicU64Array, "u64", u64, AtomicU64);
}

#[cfg(feature = "integer_atomics")]
use self::integer::*;

impl_array!(AtomicUsizeArray, "usize", usize, AtomicUsize);
impl_array!(AtomicIsizeArray, "isize", isize, AtomicIsize);
