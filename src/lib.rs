//! # orx-pinned-vec
//!
//! `PinnedVec` trait defines the interface for vectors which guarantee that elements are pinned to their memory locations with the aim to enable convenient self-referential collections.
//!
//! ## A. Implementations
//!
//! A `PinnedVec` guarantees that positions of its elements are not changed implicitly. Note that `std::vec::Vec` does not satisfy this requirement.
//!
//! [`SplitVec`](https://crates.io/crates/orx-split-vec) and [`FixedVec`](https://crates.io/crates/orx-fixed-vec) are two efficient implementations.
//!
//! ## B. Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code, following [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) could be useful for the interested.
//! * It is crucial in representing self-referential types with thin references.
//!
//! This crate focuses on the latter. Particularly, it aims to make it safe and convenient to build **performant self-referential collections** such as linked lists, trees or graphs.
//!
//! As explained in rust-docs [here](https://doc.rust-lang.org/std/pin/index.html), there exist types `Pin` and `Unpin` for this very purpose. Through the theoretical discussions, one can easily agree on the safety. However, the solution is complicated with all words `PhantomPinned`, `NonNull`, `dangling`, `Box::pin`, etc. which are alien to the self-referential data structures we are trying to build.
//!
//! This crate suggests the following approach:
//!
//! * Instances of the self-referential type will be collected together in a vector.
//! * Referencing each other will be through the natural `&` way rather than requiring any of the smart pointers.
//! * In terms of convenience, building the collection will be close to building a regular vector.
//!
//! ## C. Self-Referential-Collection Element Traits
//!
//! This crate also defines under the `orx_pinned_vec::self_referential_elements` module the required traits to enable building self referential collections with thin references.
//!
//! * `SelfRefNext` trait simply requires:
//!   * `fn next(&self) -> Option<&'a Self>;` and
//!   * `fn set_next(&mut self, next: Option<&'a Self>);` methods.
//!
//! `SelfRefPrev` is the previous counterpart.
//!
//! Notice that these two traits are sufficient to define a linked list. [`orx_linked_list::LinkedList`](https://crates.io/crates/orx-linked-list) implements `SelfRefPrev` and `SelfRefNext` to conveniently define a recurisve doubly linked list.
//!
//! Further, there exist multiple reference counterparts. They are useful in defining relations such as the *children* of a tree node or *heads of outgoing arcs* from a graph node, etc. There exist *vec* variants to be used for holding variable number of references. However, there also exist constant sized array versions which are useful in structures such as binary search trees where the number of references is bounded by a const.
//!
//! The table below presents the complete list of traits which suffice to define all aforementioned relations:
//!
//! |                                             | prev           | next           |
//! |---------------------------------------------|----------------|----------------|
//! | single reference                            | SelfRefPrev    | SelfRefNext    |
//! | dynamic number of references                | SelfRefPrevVec | SelfRefNextVec |
//! | multiple elements with a `const` max-length | SelfRefPrevArr | SelfRefNextArr |
//!
//! ## D. Safety
//!
//! With self referential collections, some mutating methods can lead to critical problems. These are the methods which change positions of already pushed elements or remove elements from the vector:
//!
//! * `insert`
//! * `remove`
//! * `pop`
//! * `swap`
//! * `truncate`
//!
//! These methods can invalidate the references among elements. Therefore, `PinnedVec` defines them as **unsafe**. One exception is the `clear` method which is safe since all elements are removed together with their references at once.
//!
//! In addition, `clone` method as well is **unsafe**, since the elements of the clone would be referencing the elements of the original vector.
//!
//! These are due to the fact that, naive implementations would cause false references. This does not mean that it is not possible to provide a safe implementation. Instead, it means that each data structure would need a different implementation (insert method of a tree and that of a linked-list cannot be implemented in the same way, they will need to update references differently).
//!
//! Implementors can provide careful safe implementations, such as `orx_linked_list::LinkedList` safely implement `Clone`, although it uses any `PinnedVec` as the underlying storage.
//!
//! There are a few cases other than self referencing collections, where a `PinnedVec` is useful. And there is no reason to treat these methods as unsafe if the elements are not referencing each other. For this purpose, `NotSelfRefVecItem` marker trait is defined. This trait works as follows:
//!
//! * if `V` implements `PinnedVec<T>`, and
//! * if `T` implements the marker trait `NotSelfRefVecItem`,
//! * => then, `V` also implements `PinnedVecSimple<T>` which provides the safe versions of the abovementioned methods.
//!
//! `NotSelfRefVecItem` trait is implemented for most primitives; however, one needs to implement for new types to explicitly state that the type is <ins>not</ins> self-referential.
//!
//! ## E. Relation with the `ImpVec`
//!
//! Providing pinned memory location elements with `PinnedVec` is the first block for building self referential structures; the second building block is the [`ImpVec`](https://crates.io/crates/orx-imp-vec). An `ImpVec` wraps any `PinnedVec` implementation and provides specialized methods built on the pinned element guarantee in order to allow building self referential collections.
//!
//! ## License
//!
//! This library is licensed under MIT license. See LICENSE for details.

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

mod not_self_ref;
mod pinned_vec;
mod pinned_vec_simple;
/// Traits to define variants of self-referential-collection elements.
pub mod self_referential_elements;
/// Utility functions to make PinnedVec implementations more convenient.
pub mod utils;

pub use not_self_ref::NotSelfRefVecItem;
pub use pinned_vec::PinnedVec;
pub use pinned_vec_simple::PinnedVecSimple;
