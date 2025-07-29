# orx-pinned-vec

[![orx-pinned-vec crate](https://img.shields.io/crates/v/orx-pinned-vec.svg)](https://crates.io/crates/orx-pinned-vec)
[![orx-pinned-vec crate](https://img.shields.io/crates/d/orx-pinned-vec.svg)](https://crates.io/crates/orx-pinned-vec)
[![orx-pinned-vec documentation](https://docs.rs/orx-pinned-vec/badge.svg)](https://docs.rs/orx-pinned-vec)

`PinnedVec` trait defines the interface for vectors which guarantee that elements added to the vector are pinned to their memory locations unless explicitly changed.

> This crate is **no-std** by default.

## Pinned Elements Guarantees

A `PinnedVec` guarantees that positions of its elements **do not change implicitly**.

Assume that we have a pinned vector which is at a state containing **n** elements. Pinned vector guarantees can then be summarized as follows.

* **G1: pinned elements on growth at the end**. This is one of the critical guarantees that the pinned vectors provide. We are adding **m** ≥ 1 elements to the end of the vector, and hence the vector reaches a length of **n + m**. Pinned vector guarantees that the memory locations of the **n** elements will not change due to this mutation.
  * *Some such example mutation methods are **push**, **extend** or **extend_from_slice**.*
  * *However, **insert** method is not covered since it is not an addition to the end of the vector.*
  * *Notice that the standard vector does not satisfy this requirement.*
  * *For many special data structures, such as concurrent collections or self referential collections, this is the necessary and sufficient pinned element guarantee.*
* **G2: pinned elements on removals from the end**. In this case, we are removing **m** ∈ [1, n] elements from the end of the vector leading to the final vector length of **n - m**. Pinned vector guarantees that memory locations of these remaining **n - m** elements do not change.
  * *Some such example methods are **pop**, **truncate** or **clear**.*
* **G3: pinned prior elements on insertions to arbitrary position**. Assume we are adding **m** ≥ 1 elements; however, not necessarily to the end of the vector this time. The earliest position of the inserted elements is **p** < (n-1). In this case, pinned vector guarantees that memory locations of the elements at positions 0..(p-1) will remain intact.
  * *The example method is the **insert** method.*
* **G4: pinned prior elements in removals from arbitrary position**. Lastly, assume that we are removing **m** ∈ [1, n] elements from the arbitrary positions of the vector leading to a final vector length of **n - m**. Let **p** be the earliest position of the removed elements. Pinned vector then guarantees that memory locations of the elements at positions 0..(p-1) will remain intact.
  * *The example method is the **remove** method.*

## Motivation & Examples

Note that this eliminates a certain set of errors that are easy to make in some languages and forbidden by the borrow checker in rust. Consider, for example, the classical example that does not compile in rust. The reason this code has a bug is due to the fact that the elements of the standard vector are not pinned to their memory locations and it is possible that the `push` leads to changing them all together. Using a pinned vector, on the other hand, this would be a memory safe operation.

```rust
let mut vec = vec![0, 1, 2, 3];

let ref_to_first = &vec[0];
assert_eq!(ref_to_first, &0);

vec.push(4);

// does not compile due to the following reason:  cannot borrow `vec` as mutable because it is also borrowed as immutable
// assert_eq!(ref_to_first, &0);
```

An example data structure relying on pinned vector guarantees to enable immutable push operation is the [ImpVec](https://crates.io/crates/orx-imp-vec) which is also discussed in this [article](https://orxfun.github.io/orxfun-notes/#/imp-vec-motivation-2024-10-03).

Furthermore, self-referential-collections following thin references rather than wide pointers or index numbers rely on the consistency of memory positions of its elements. Pinned vectors again come in very useful for them. You may find efficient [LinkedList](https://crates.io/crates/orx-linked-list) and [Tree](https://crates.io/crates/orx-tree) implementations built on top of the pinned element guarantees.

Finally, it is easy to observe that pinned element guarantees make it extremely more convenient and safer to achieve data structures which enable concurrent growth. There are various concurrent data structures relying on pinned vectors, such as [ConcurrentVec](https://crates.io/crates/orx-concurrent-vec), [ConcurrentBag](https://crates.io/crates/orx-concurrent-bag) and [ConcurrentOrderedBag](https://crates.io/crates/orx-concurrent-ordered-bag). These concurrent collections play an essential role in efficiently collecting results of a parallel computation by the parallel iterator [Par](https://crates.io/crates/orx-parallel).

## Testing the Guarantees

`PinnedVec` trait on its own cannot provide the pinned element guarantee; hence, it could be considered as a marker trait.

However, this crate additionally provides the test function to assert these guarantees:

```rust ignore
pub fn test_pinned_vec<P: PinnedVec<usize>>(pinned_vec: P, test_vec_len: usize) {
    // ...
}
```

This function performs an extensive test on the specific implementation `P` and fails if any of the above guarantees is not provided.

Note that `std::vec::Vec` does not provide the pinned elements during growth guarantee. You may find a wrapper struct `JustVec` which is nothing but the standard vec here: [src/pinned_vec_tests/test_all.rs](https://github.com/orxfun/orx-pinned-vec/blob/main/src/pinned_vec_tests/test_all.rs). As expected, `test_pinned_vec` method fails for this struct.

## Implementations

[`SplitVec`](https://crates.io/crates/orx-split-vec) and [`FixedVec`](https://crates.io/crates/orx-fixed-vec) are two efficient PinnedVec implementations.

## Contributing

Contributions are welcome! If you notice an error, have a question or think something could be improved, please open an [issue](https://github.com/orxfun/orx-pinned-vec/issues/new) or create a PR.

## License

Dual-licensed under [Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT).
