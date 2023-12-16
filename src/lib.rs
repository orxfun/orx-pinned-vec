//! # orx-pinned-vec
//!
//! `PinnedVec` trait defines the interface for vectors which guarantee that elements are pinned to their memory locations unless explicitly changed by the caller, with the aim to enable convenient self-referential collections.
//!
//! ## A. Implementations
//!
//! A `PinnedVec` guarantees that positions of its elements are not changed implicitly. Note that `std::vec::Vec` does not satisfy this requirement.
//!
//!  [`FixedVec`](https://crates.io/crates/orx-fixed-vec) and [`SplitVec`](https://crates.io/crates/orx-split-vec) are two example implementations.
//!
//! ## B. Motivation
//!
//! There might be various situations where pinned elements are helpful.
//!
//! * It is somehow required for async code. Not being an expert in the subject, leaving this [blog](https://blog.cloudflare.com/pin-and-unpin-in-rust) for the interested.
//! * It is a requirement to make self-referential types possible.
//!
//! This crate focuses more on the latter. Particularly, it aims to make it safely and conveniently possible to build **self-referential collections** such as linked list, tree or graph.
//!
//! As explained in rust-docs [here](https://doc.rust-lang.org/std/pin/index.html), there exists types `Pin` and `Unpin` for this very purpose. Through the theoretical discussions, one can easily agree on the safety. However, it is hard to consider the solution as a convenient one with all words `PhantomPinned`, `NonNull`, `dangling`, `Box::pin`, etc. which are alien to the self-referential data structures we are trying to build.
//!
//! This crate suggests the following approach:
//!
//! * Instances of the self-referential type will be collected together in a vector.
//! * Referencing each other will be through the natural `&` way rather than requiring any of the smart pointers.
//! * In terms of convenience, building the collection will be close to building a regular vector.
//!
//!
//! ## C. Safety
//!
//! In order to safely build a self-referential collection, we have two requirements
//!
//! * allowing for immutable growth, and
//! * disallowing non-growth structural mutations.
//!
//! To give concrete examples to the arguments, we will use an example throughout the section. Assume we would like to build a tree where each node keeps a vector of references to its children (we omit the parent for brevity). Our tree will simply be a vector. Each node will be an element of this vector. However, vector is nothing but a storage. We will have our regular tree traversals through references.
//!
//! We will need the index method and the push method of the vector:
//!
//! * `fn push(&mut self, element: T)`.
//!
//! The node will simply look as below, free of alien words. We need to add the lifetimes `'a`. However, all lifetimes will trivially be equal to each other since we know that the nodes will live together as elements of the same vector.
//!
//! ```rust
//! struct Node<'a, T> {
//!     data: T,
//!     children: Vec<&'a Node<'a, T>>,
//! }
//!
//! impl<'a, T> Node<'a, T> {
//!     fn new(data: T, children: Vec<&'a Node<'a, T>>) -> Self {
//!         Self { data, children }
//!     }
//! }
//! ```
//!
//! Finally, assume the tree we are trying to build is:
//!
//! ```text
//!       a
//!      / \
//!     b   c
//!     |
//!     d
//! ```
//!
//!
//! ## C.1. Safety - Immutable Push
//!
//! We need to be able to push to the vector with an immutable reference. It might (will) sound counter-intuitive. We will first discuss why it is necessary and then why it is okay (opinionated).
//!
//! We immediately notice that we cannot start building the tree from the root 'a':
//!
//! ```rust
//! # struct Node<'a, T> {
//! #     data: T,
//! #     children: Vec<&'a Node<'a, T>>,
//! # }
//! #
//! # impl<'a, T> Node<'a, T> {
//! #     fn new(data: T, children: Vec<&'a Node<'a, T>>) -> Self {
//! #         Self { data, children }
//! #     }
//! # }
//! let mut tree = vec![];
//!
//! let root = Node::new('a', vec![]); // ? root's children b and c do not exist yet!
//! tree.push(root);
//! ```
//!
//! So we start from the leaves:
//!
//! ```rust
//! # struct Node<'a, T> {
//! #     data: T,
//! #     children: Vec<&'a Node<'a, T>>,
//! # }
//! #
//! # impl<'a, T> Node<'a, T> {
//! #     fn new(data: T, children: Vec<&'a Node<'a, T>>) -> Self {
//! #         Self { data, children }
//! #     }
//! # }
//! let mut tree = vec![];
//!
//! let d = Node::new('d', vec![]);
//! tree.push(d);
//!
//! let ref_d = &tree[0];
//! let b = Node::new('b', vec![ref_d]); // so far so good
//!
//! // tree.push(b); // this line won't compile!
//! ```
//!
//! According to the borrow checker, the reason we cannot push is that we are holding a reference to the `tree` which is `ref_d`, and we cannot mutate the tree unless we drop this reference.
//!
//! But why?
//!
//! There are two reasons:
//!
//! 1. **Correctness:** First, we are mutating a structure while holding a reference to it. This is bad, it might lead to wrong assumptions and bugs.
//! 2. **Undefined Behavior:** Second, the underlying data of the vector can be copied around while increasing the vector capacity. This could cause `ref_d` to be a dangling reference.
//!
//! Seperating these two issues is useful in understanding the reasons and providing a solution.
//!
//! ### C.1.a - Correctness
//!
//! Here we ignore the dangling reference issue, as it will be discussed separately.
//!
//! How bad of a mutation is pushing an element to a vector? Priorly defined and pushed elements will not change. Why is it different than defining a new variable in the scope?
//!
//! There is actually one problem, aggregations. By pushing a new element to the vector:
//!
//! * we change its `len`,
//! * we change the average value of some numeric property of its elements,
//! * etc.
//!
//! If we rely on the values of these functions to remain unchanged without a `mut` reference, we would be wrong. On the other hand, such a `push` operation with an immutable reference fits well the builders or constructors where we keep creating things rather than performing computations on the created ones. This is the goal we want to achieve here.
//!
//! ### C.1.b - Undefined Behavior
//!
//! This is very critical. Therefore, we must make sure that `push` operation does <ins>not</ins> cause implicit moves of the data. Every `PinnedVec` must provide this guarantee that all elements will be **pinned** while growing with methods such as push or extend.
//!
//! [`FixedVec`](https://crates.io/crates/orx-fixed-vec) and [`SplitVec`](https://crates.io/crates/orx-split-vec) are two examples providing these guarantees.
//!
//! ### C.1.c - Building Acyclic Self-Referential Collections with Imp
//!
//! As the requirement is established, assume that our hypothetical `Vec` provides us the immutable push method `fn imp(&self, element: T)`. Then, the following code would compile and allow us to build our tree.
//!
//! ```rust ignore
//! let tree = vec![];
//!
//! tree.imp(Node::new('d', vec![]));
//! let ref_d = &tree[0];
//!
//! tree.imp(Node::new('b', vec![ref_d]));
//! let ref_b = &tree[1];
//!
//! tree.imp(Node::new('c', vec![]));
//! let ref_c = &tree[2];
//!
//! tree.imp(Node::new('a', vec![ref_b, ref_c]));
//! ```
//!
//! This is already significantly simpler than alternatives such as `Pin`. With some work, we can get rid of the indexed-accesses and make the code more concise and less error prone (see [push_get_ref](https://docs.rs/orx-imp-vec/0.9.7/orx_imp_vec/struct.ImpVec.html#method.push_get_ref) method of [ImpVec](https://crates.io/crates/orx-imp-vec)).
//!
//! Actually, immutable push method is sufficient to very easily build all acyclic self-referential collections such as directed acyclic graphs. It also covers the tree here.
//!
//! When the collection has cyclic relations, such as regular trees where nodes have both children and parent information, or cyclic graphs, or linked lists, we need a little bit more book keeping. However, immutable push still provides the main convenience to build such data structures. See [orx_linked_list::LinkedList](https://crates.io/crates/orx-linked-list) for an example.
//!
//! ### C.1.d - Relation with [ImpVec](https://crates.io/crates/orx-imp-vec)
//!
//! Note that `PinnedVec` does <ins>not</ins> provide the immutable push method. It only provides the required behavior.
//!
//! Any `V` implementing `PinnedVec` can be wrapped by `ImpVec`. While wrapped, `ImpVec` safely allows the immutable push operation relying on the guarantees by the `PinnedVec`.
//!
//! This relation allows for a clear separation of the building stage. For instance, one may
//!
//! * create an empty `V: PinnedVec` (initiate)
//! * wrap it in `ImpVec` (enter building phase)
//! * build the self referential collection using the immutable push operation (build)
//! * unwrap the `ImpVec` and get back the built data structure as `V` (leave the building phase)
//!
//! ## C.2. Safety - Non-growing Mutations & Clone
//!
//! With self referential collections, some other mutating methods can lead to critical problems as well. These are the methods which change positions of already pushed elements or remove elements from the vector:
//!
//! * `insert`
//! * `remove`
//! * `pop`
//! * `swap`
//! * `truncate`
//!
//! These methods can invalidate the references among elements. Therefore, `PinnedVec` defines them as **unsafe**. One exception is the `clear` method which is safe since all elements are removed together with their references at once.
//!
//! Finally, `clone` method as well is **unsafe**, since the elements of the clone would be referencing the elements of the original vector.
//!
//! These are due to the fact that, naive implementations would cause false references. This does not mean that it is not possible to provide a safe implementation. Instead, it means that each data structure would need a different implementation (insert of a tree and linked-list cannot be implemented in the same way).
//!
//! Implementors can provide careful safe implementations, such as `orx_linked_list::LinkedList` safely implement `Clone`, although it uses any `PinnedVec` as the underlying storage.
//!
//! Further, there are a few cases other than self referencing collections, where a `PinnedVec` is useful. And there is no reason to treat these methods as unsafe if the elements are not referencing each other. For this purpose, `NotSelfRefVecItem` marker trait is defined. This trait works as follows:
//!
//! * if `V` implements `PinnedVec<T>`, and
//! * if `T` implements the marker trait `NotSelfRefVecItem`,
//! * => then, `V` also implements `PinnedVecSimple<T>` which provides the safe versions of the abovementioned methods.
//!
//! `NotSelfRefVecItem` trait is implemented for most primitives; however, one needs to implement for new types to explicitly state that the type is <ins>not</ins> self-referential.
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
mod self_ref;
/// Utility functions to make PinnedVec implementations more convenient.
pub mod utils;

pub use not_self_ref::NotSelfRefVecItem;
pub use pinned_vec::PinnedVec;
pub use pinned_vec_simple::PinnedVecSimple;
pub use self_ref::SelfRefVecItem;
