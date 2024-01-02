use std::fmt::{Debug, Formatter, Result};

/// Trait for vector representations differing from `std::vec::Vec` by the following:
///
/// => memory location of an element already pushed to the collection never changes unless any of the following mut-methods is called:
///
/// * `clear`, `resize`, `truncate`,
/// * `insert`
/// * `remove`, `pop`.
///
/// Equivalently:
///
/// => the mut-methods `push` or `extend_from_slice` do <ins>not</ins> change memory locations of priorly added elements.
///
/// This pinned property is crticial and required for building collections such as [`orx-imp-vec::ImpVec`](https://crates.io/crates/orx-imp-vec).
///
/// # Safety: Pinned Elements
///
/// Note that, together with `PinnedVecSimple`, this trait defines the required signature for implementing pinned vectors.
/// However, the implementor struct is responsible for guaranteeing that `push` and `extend_from_slice` do not change memory locations
/// of already added elements.
///
/// As expected, `push` and `extend_from_slice` are mut-methods. However, when the `PinnedVec` with pinned guarantees is
/// wrapped in an `ImpVec`, these methods require only `&self` allowing to safely and conveniently represent complex-for-rust
/// data structures.
///
/// # Safety: Self Referential Vectors
///
/// One of the two main goals of the target `ImpVec` is to make it safe and convenient to build self-referential data structures,
/// such as linked lists or trees. Elements of the vector being self-referential or not has a critical impact on safety of the
/// mut-methods.
///
/// *The arguments below are based on the possibility of self-referential vectors where elements can hold references to each other.
/// This is not common in rust; however, it is safely possible with data structures such as `ImpVec`.*
///
/// For instance, as expected `insert` is a safe method if the elements of the vector do not hold references to each other. However,
/// if they do, since `insert` will change memory locations of existing elements, this operation is `unsafe`.
///
/// Therefore, the distinction is provided by the trait `NotSelfRefVecItem`.
///
/// For `PinnedVec<T>`
/// * all mut-methods which change the memory locations of existing items such as `insert`, `remove` and `swap` are `unsafe`
/// since this would invalidate the references held by other elements,
/// * all mut-methods which remove elements from the vector such as `pop`, `truncate` or `remove` are `unsafe`
/// since there might be elements holding a reference to the removed items,
///   * with the exception that `clear` is safe since all elements are gone at once,
/// * additionally `clone` is `unsafe` because the cloned elements would be holding references to the original vector.
///
/// However, all these methods become safe if `T: NotSelfRefVecItem` through the type system as follows:
///
/// * every `V: PinnedVec<T>` also automatically implements `PinnedVecSimple<V>`,
/// * and `PinnedVecSimple<V>` provides safe versions of the above mentioned methods.
///
/// This makes sense since all the mentioned unsafety stems from the possibility of elements holding references to each other.
///
/// Note that `NotSelfRefVecItem` is a marker trait which means that the implementor takes the responsibility.
/// Further, it is implemented for most primitive types.
pub trait PinnedVec<T> {
    /// Iterator yielding references to the elements of the vector.
    type Iter<'a>: Iterator<Item = &'a T>
    where
        T: 'a,
        Self: 'a;
    /// Iterator yielding mutable references to the elements of the vector.
    type IterMut<'a>: Iterator<Item = &'a mut T>
    where
        T: 'a,
        Self: 'a;

    // pinned
    /// Returns the index of the `element` with the given reference.
    ///
    /// Note that `T: Eq` is not required; reference equality is used.
    ///
    /// The complexity of this method depends on the particular `PinnedVec` implementation.
    /// However, making use of referential equality, it ispossible to perform much better than *O(n)*,
    /// where n is the vector length.
    ///
    /// For the two example implementations, complexity of this method:
    /// * *O(1)* for [FixedVec](https://crates.io/crates/orx-fixed-vec);
    /// * *O(f)* for [SplitVec](https://crates.io/crates/orx-split-vec) where f << n is the number of fragments.
    fn index_of(&self, data: &T) -> Option<usize>;

    // vec
    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    ///
    /// # Safety
    ///
    /// `clear` operation is **safe** both when `T: NotSelfRefVecItem` or not due to the following:
    ///
    /// * elements holding references to each other will be cleaned all together; hence,
    /// none of them can have an invalid reference;
    /// * we cannot keep holding a reference to a vector element defined aliased the `clear` call,
    /// since `clear` requires a `mut` reference.
    fn clear(&mut self);
    /// Returns the total number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;
    /// Clones and appends all elements in a slice to the Vec.
    ///
    /// Iterates over `other`, clones each element, and then appends it to this vec. The other slice is traversed in-order.
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone;

    /// Returns a reference to an element with the given `index` returns None if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&T>;
    /// Returns a mutable reference to an element with the given `index` returns None if the index is out of bounds.
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    /// Returns a reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see `get`.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(&self, index: usize) -> &T;
    /// Returns a mutable reference to an element without doing bounds checking.
    ///
    /// For a safe alternative see `get_mut`.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

    /// Returns true if the vector contains no elements.
    fn is_empty(&self) -> bool;
    /// Returns the number of elements in the vector, also referred to as its length.
    fn len(&self) -> usize;
    /// Appends an element to the back of a collection.
    fn push(&mut self, value: T);

    // vec but unsafe
    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    ///
    /// `insert` is unsafe since insertion of a new element at an arbitrary position of the vector
    /// changes the positions or memory locations of already existing elements.
    /// This is considered **unsafe** since it violates `PinnedVec` guarantees.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_insert(&mut self, index: usize, element: T);
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is <ins>not</ins> `NotSelfRefVecItem`.
    ///
    /// `remove` is unsafe since removal of an element at an arbitrary position of the vector
    /// changes the positions of already existing elements.
    /// This is considered **unsafe** since it violates `PinnedVec` guarantees.
    ///
    /// Further, it is possible that one of the remaining elements is
    /// pointing to the element which is being removed.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_remove(&mut self, index: usize) -> T;
    /// Removes the last element from a vector and returns it, or None if it is empty.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    ///
    /// `pop` is unsafe since it is possible that one of the remaining elements is holding a
    /// reference to the last element which is being popped.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_pop(&mut self) -> Option<T>;
    /// Swaps two elements in the slice.
    ///
    /// If `a` equals to `b`, it's guaranteed that elements won't change value.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    ///
    /// `swap` is unsafe since it is possible that other elements are referencing the
    /// elements to be swapped. The swap operation would invalidate these references,
    /// pointing at wrong elements.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_swap(&mut self, a: usize, b: usize);
    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    ///
    /// `truncate` is unsafe since it is possible that remaining elements are referencing
    /// to elements which are dropped by the truncate method.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_truncate(&mut self, len: usize);
    /// Creates and returns a clone of the vector.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    ///
    /// To understand why `clone` is unsafe when `T` is not `NotSelfRefVecItem`,
    /// consider the following example.
    ///
    /// ```rust ignore
    /// let vec = ...;  // vec is a self-referential PinnedVec, equivalently,
    ///                 // it is a `PinnedVec<T>` where T is not `NotSelfRefVecItem`
    /// let clone = vec.clone();
    /// ```
    ///
    /// Now the elements of `clone` are holding references to the `vec` which is bad!
    ///
    /// * these references are meant to be kept internal to the vector, so is the name self-ref,
    /// * further, if `vec` is dropped, `clone` elements will be holding invalid references leading to UB.
    ///
    /// `PinnedVec` takes the conservative approach:
    /// every T which does <ins>not</ins> implement the marker trait `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// On the other hand, safe version of this method is available for `T: NotSelfRefVecItem`.
    unsafe fn unsafe_clone(&self) -> Self
    where
        T: Clone;

    /// Returns an iterator to elements of the vector.
    fn iter(&self) -> Self::Iter<'_>;
    /// Returns an iterator of mutable references to elements of the vector.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    // required for common trait implementations
    /// This method tests for `self` and `other` values to be equal, and is used by `==`.
    fn partial_eq<S>(&self, other: S) -> bool
    where
        S: AsRef<[T]>,
        T: PartialEq;
    /// Formats the value using the given formatter.
    fn debug(&self, f: &mut Formatter<'_>) -> Result
    where
        T: Debug;
}
