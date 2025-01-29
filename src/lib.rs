#![doc = include_str!("../README.md")]
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
#![no_std]

extern crate alloc;

mod capacity;
mod concurrent_pinned_vec;
mod errors;
mod into_concurrent_pinned_vec;
mod pinned_vec;
/// Tests methods to validate pinned element guarantees of an implementing type.
pub mod pinned_vec_tests;
/// Utility functions to make PinnedVec implementations more convenient.
pub mod utils;

pub use capacity::CapacityState;
pub use concurrent_pinned_vec::ConcurrentPinnedVec;
pub use errors::PinnedVecGrowthError;
pub use into_concurrent_pinned_vec::IntoConcurrentPinnedVec;
pub use orx_iterable::{Collection, CollectionMut, Iterable};
pub use pinned_vec::PinnedVec;
pub use pinned_vec_tests::test_pinned_vec;
