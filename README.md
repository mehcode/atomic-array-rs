# atomic-array
![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
[![Crates.io](https://img.shields.io/crates/d/atomic-array.svg)](https://crates.io/crates/atomic-array)
[![Docs.rs](https://docs.rs/atomic-array/badge.svg)](https://docs.rs/atomic-array)
> Defines several array types in which elements may be updated atomically. Intended to provide atomic array types similar to those found in `java.util.concurrent.atomic` in Java.

Provides the following types:

 * `AtomicOptionRefArray` – Corresponds to [`AtomicReferenceArray`](https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/atomic/AtomicReferenceArray.html).
 * `AtomicRefArray` – `AtomicOptionRefArray` with enforced default values to remove the optional property of the elements.
 * `AtomicBoolArray`
 * `AtomicUsizeArray`, `AtomicIsizeArray`
 * `AtomicI8` ... `AtomicI64Array` (requires a `nightly` compiler and `integer_atomics` feature)
 * `AtomicU8` ... `AtomicU64Array` (requires a `nightly` compiler and `integer_atomics` feature)

## Usage

```toml
[dependencies]
atomic-array = "0.3"
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
