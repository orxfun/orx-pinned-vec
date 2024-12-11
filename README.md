# orx-pinned-vec

[![orx-pinned-vec crate](https://img.shields.io/crates/v/orx-pinned-vec.svg)](https://crates.io/crates/orx-pinned-vec)
[![orx-pinned-vec documentation](https://docs.rs/orx-pinned-vec/badge.svg)](https://docs.rs/orx-pinned-vec)

`PinnedVec` trait defines the interface for vectors which guarantee that elements added to the vector are pinned to their memory locations unless explicitly changed.

## A. Pinned Elements Guarantee

A `PinnedVec` guarantees that positions of its elements **do not change implicitly**.

To be specific, let's assume that a pinned vector currently has `n` elements:

| Method    | Expected Behavior |
| -------- | ------- |
| `push(new_element)` | does not change the memory locations of the `n` elements |
| `extend_from_slice(slice)` | does not change the memory locations of the first `n` elements |
| `insert(a, new_element)` | does not change the memory locations of the first `a` elements, where `a <= n`; elements to the right of the inserted element might be changed, commonly shifted to right |
| `pop()` | does not change the memory locations of the first `n-1` elements, the `n`-th element is removed |
| `remove(a)` | does not change the memory locations of the first `a` elements, where `a < n`; elements to the right of the removed element might be changed, commonly shifted to left |
| `truncate(a)` | does not change the memory locations of the first `a` elements, where `a < n` |
||

Note that this eliminates a certain set of errors that are easy to make in some languages and forbidden by the borrow checker in rust. Consider, for example, the classical example that does not compile in rust. The reason this code has a bug is due to the fact that the elements of the standard vector are not pinned to their memory locations and it is possible that the `push` leads to changing them all together. Using a pinned vector, on the other hand, this would be a memory safe operation.

```rust
let mut vec = vec![0, 1, 2, 3];

let ref_to_first = &vec[0];
assert_eq!(ref_to_first, &0);

vec.push(4);

// does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
// assert_eq!(ref_to_first, &0);
```


`PinnedVec` trait on its own cannot provide the pinned element guarantee; hence, it could be considered as a marker trait.

However, this crate additionally provides the test function to assert these guarantees:

```rust ignore
pub fn test_pinned_vec<P: PinnedVec<usize>>(pinned_vec: P, test_vec_len: usize) {
    // ...
}
```

This function performs an extensive test on the specific implementation `P` and fails if any of the above guarantees is not provided.

Note that `std::vec::Vec` does not provide the pinned elements during growth guarantee. You may find a wrapper struct `JustVec` which is nothing but the standard vec here: [src/pinned_vec_tests/test_all.rs](https://github.com/orxfun/orx-pinned-vec/blob/main/src/pinned_vec_tests/test_all.rs). As expected, `test_pinned_vec` method fails for this struct.

## B. Motivation

There are various situations where pinned elements are critical.

* It is critical in enabling **efficient, convenient and safe self-referential collections** with thin references, see [`SelfRefCol`](https://crates.io/crates/orx-selfref-col) for details, and its special cases such as [`LinkedList`](https://crates.io/crates/orx-linked-list).
* It is important for **concurrent** programs as it eliminates safety concerns related with elements implicitly carried to different memory locations. This helps reducing and dealing with the complexity of concurrency, and leads to efficient concurrent data structures. See [`ConcurrentIter`](https://crates.io/crates/orx-concurrent-iter), [`ConcurrentBag`](https://crates.io/crates/orx-concurrent-bag) or [`ConcurrentOrderedBag`](https://crates.io/crates/orx-concurrent-ordered-bag) for such concurrent data structures which are conveniently built on the pinned element guarantees of pinned vectors.
* It is essential in allowing an **immutable push** vector; i.e., [`ImpVec`](https://crates.io/crates/orx-imp-vec). This is a very useful operation when the desired collection is a bag or a container of things, rather than having a collective meaning. In such cases, `ImpVec` allows avoiding certain borrow checker complexities, heap allocations and wide pointers such as `Box` or `Rc` or etc.

## C. Implementations

[`SplitVec`](https://crates.io/crates/orx-split-vec) and [`FixedVec`](https://crates.io/crates/orx-fixed-vec) are two efficient implementations.

## License

This library is licensed under MIT license. See LICENSE for details.
