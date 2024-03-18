/// Trait for vector representations differing from `std::vec::Vec` by the following:
///
/// => memory location of an element already pushed to the collection never changes unless any of the following mut-methods is called:
/// * `remove`, `pop`,
/// * `insert`,
/// * `clear`, `truncate`.
///
/// In other words,
///
/// => the mut-methods `push` or `extend_from_slice` do <ins>not</ins> change memory locations of already added elements.
///
/// # Pinned Elements Guarantee
///
/// A `PinnedVec` guarantees that positions of its elements **do not change implicitly**.
///
/// To be specific, let's assume that a pinned vector currently has `n` elements:
///
/// | Method    | Expected Behavior |
/// | -------- | ------- |
/// | `push(new_element)` | does not change the memory locations of the `n` elements |
/// | `extend_from_slice(slice)` | does not change the memory locations of the first `n` elements |
/// | `insert(a, new_element)` | does not change the memory locations of the first `a` elements, where `a <= n`; elements to the right of the inserted element might be changed, commonly shifted to right |
/// | `pop()` | does not change the memory locations of the first `n-1` elements, the `n`-th element is removed |
/// | `remove(a)` | does not change the memory locations of the first `a` elements, where `a < n`; elements to the right of the removed element might be changed, commonly shifted to left |
/// | `truncate(a)` | does not change the memory locations of the first `a` elements, where `a < n` |
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
    /// Iterator yielding references to the elements of the vector.
    type IterRev<'a>: Iterator<Item = &'a T>
    where
        T: 'a,
        Self: 'a;
    /// Iterator yielding mutable references to the elements of the vector.
    type IterMutRev<'a>: Iterator<Item = &'a mut T>
    where
        T: 'a,
        Self: 'a;

    // pinned
    /// Returns the index of the `element` with the given reference.
    ///
    /// Note that `T: Eq` is not required; reference equality is used.
    ///
    /// The complexity of this method depends on the particular `PinnedVec` implementation.
    /// However, making use of referential equality, it possible to perform much better than *O(n)*,
    /// where n is the vector length.
    ///
    /// For the two example implementations, complexity of this method:
    /// * *O(1)* for [FixedVec](https://crates.io/crates/orx-fixed-vec);
    /// * *O(f)* for [SplitVec](https://crates.io/crates/orx-split-vec) where f << n is the number of fragments.
    fn index_of(&self, element: &T) -> Option<usize>;

    /// Returns whether or not of the `element` with the given reference belongs to this vector.
    /// In other words, returns whether or not the reference to the `element` is valid.
    ///
    /// Note that `T: Eq` is not required; memory address is used.
    ///
    /// The complexity of this method depends on the particular `PinnedVec` implementation.
    /// However, making use of pinned element guarantees, it possible to perform much better than *O(n)*,
    /// where n is the vector length.
    ///
    /// For the two example implementations, complexity of this method:
    /// * *O(1)* for [FixedVec](https://crates.io/crates/orx-fixed-vec);
    /// * *O(f)* for [SplitVec](https://crates.io/crates/orx-split-vec) where f << n is the number of fragments.
    fn contains_reference(&self, element: &T) -> bool;

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

    /// Returns a reference to the first element of the vector; returns None if the vector is empty.
    fn first(&self) -> Option<&T>;
    /// Returns a reference to the last element of the vector; returns None if the vector is empty.
    fn last(&self) -> Option<&T>;

    /// Returns a reference to the first element of the vector without bounds checking.
    ///
    /// For a safe alternative see `first`.
    ///
    /// # Safety
    ///
    /// Calling this method when the vector is empty is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn first_unchecked(&self) -> &T;
    /// Returns a reference to the last element of the vector without bounds checking.
    ///
    /// For a safe alternative see `last`.
    ///
    /// # Safety
    ///
    /// Calling this method when the vector is empty is *[undefined behavior]* even if the resulting reference is not used.
    unsafe fn last_unchecked(&self) -> &T;

    /// Returns true if the vector contains no elements.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of elements in the vector, also referred to as its length.
    fn len(&self) -> usize;
    /// Appends an element to the back of a collection.
    fn push(&mut self, value: T);

    // vec but unsafe
    /// Inserts an element at position `index` within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    fn insert(&mut self, index: usize, element: T);
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    fn remove(&mut self, index: usize) -> T;
    /// Removes the last element from a vector and returns it, or None if it is empty.
    fn pop(&mut self) -> Option<T>;
    /// Swaps two elements in the slice.
    ///
    /// If `a` equals to `b`, it's guaranteed that elements won't change value.
    ///
    /// # Arguments
    ///
    /// * a - The index of the first element
    /// * b - The index of the second element.
    fn swap(&mut self, a: usize, b: usize);
    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    fn truncate(&mut self, len: usize);

    /// Returns an iterator to elements of the vector.
    fn iter(&self) -> Self::Iter<'_>;
    /// Returns an iterator of mutable references to elements of the vector.
    fn iter_mut(&mut self) -> Self::IterMut<'_>;
    /// Returns a reversed back-to-front iterator to elements of the vector.
    fn iter_rev(&self) -> Self::IterRev<'_>;
    /// Returns a reversed back-to-front iterator mutable references to elements of the vector.
    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_>;

    /// Returns a mutable reference to the `index`-th element of the vector.
    ///
    /// Returns `None` if `index`-th position does not belong to the vector; i.e., if `index` is out of `capacity`.
    ///
    /// # Safety
    ///
    /// This method allows to write to a memory which is greater than the vector's length.
    /// On the other hand, it will never return a pointer to a memory location that the vector does not own.
    unsafe fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T>;

    /// Forces the length of the vector to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal invariants of the type.
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to [`capacity()`].
    /// - The elements at `old_len..new_len` must be initialized.
    unsafe fn set_len(&mut self, new_len: usize);
}

#[cfg(test)]
mod tests {
    use crate::{pinned_vec_tests::testvec::TestVec, PinnedVec};

    #[test]
    fn is_empty() {
        let mut vec = TestVec::new(5);
        assert!(vec.is_empty());

        vec.push(1);
        assert!(!vec.is_empty());

        vec.push(2);
        vec.push(3);
        assert!(!vec.is_empty());

        vec.clear();
        assert!(vec.is_empty());
    }
}
