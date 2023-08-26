//! `PinnedVec` trait serves as the marker with common vector functionalities
//! for vector implementations which
//!
//! * preserves the memory locations of their elements; i.e., keeps them pinned.
//!
//! This feature eliminates a specific set of errors leading to undefined behavior (UB),
//! and hence, allows to work with a more flexible borrow checker.
//!
//! Consider for instance the following code block which does not compile.
//!
//! ```
//! let mut vec = Vec::with_capacity(2);
//! vec.extend_from_slice(&[0, 1]);
//!
//! let ref0 = &vec[0];
//! vec.push(2);
//! // let value0 = *ref0; // does not compile!
//! ```
//!
//! Note that we have not removed any elements form the vector.
//! The reason why dereferencing `ref0` causes UB is:
//!
//! * the call to push element 2 to the vector requires the vector to grow,
//! * the standard vector might (or might not) carry the data to another memory location,
//! * in that case, `ref0` is invalid which causes the UB.
//!
//! `PinnedVec` implementations, on the other hand, guarantee that such moves in
//! memory locations do not happen; and hence, eliminating the cause of the UB observed here.
//!
//! See, [`FixedVec`](https://crates.io/crates/orx-fixed-vec) and [`SplitVec`](https://crates.io/crates/orx-split-vec)
//! for two basic pinned-vector implementations.
//!
//! Further, see [`ImpVec`](https://crates.io/crates/orx-imp-vec) crate which allows converting
//! any `PinnedVec` implementation into an imp-vec.
//! An imp-vec stands for immutable-push-vector, literally allowing to push to the vector with an
//! immutable reference.
//! This turns out to be a very useful opeartion, allowing to conveniently implement tricky data structures.

#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

mod pinned_vec;
pub use pinned_vec::PinnedVec;
