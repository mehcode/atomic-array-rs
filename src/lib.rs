//! Defines several array types in which elements may be updated atomically.
//! Intended to provide atomic array types similar to those found in [java.util.concurrent.atomic](https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/package-summary.html) in Java.
#[cfg_attr(feature = "integer_atomics", feature(integer_atomics))]

mod into_option_arc;
mod option_ref_array;
mod ref_array;
mod array;

pub use self::into_option_arc::IntoOptionArc;
pub use self::option_ref_array::AtomicOptionRefArray;
pub use self::ref_array::AtomicRefArray;
pub use self::array::*;
