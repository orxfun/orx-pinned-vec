use crate::CapacityState;
use core::{
    cmp::Ordering,
    ops::{Index, IndexMut, RangeBounds},
};
use orx_iterable::{Collection, CollectionMut};
use orx_pseudo_default::PseudoDefault;

/// Trait for vector representations differing from `std::vec::Vec` by the following:
///
/// => memory location of an element already pushed to the collection never changes unless any of the following `mut` methods is called:
/// * `remove`, `pop`,
/// * `insert`,
/// * `clear`, `truncate`.
///
/// In other words,
///
/// => growth methods `push` or `extend_from_slice` do <ins>not</ins> change memory locations of already added elements.
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
pub trait PinnedVec<T>:
    IntoIterator<Item = T>
    + Collection<Item = T>
    + CollectionMut<Item = T>
    + PseudoDefault
    + Index<usize, Output = T>
    + IndexMut<usize, Output = T>
{
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

    /// Iterator yielding slices corresponding to a range of indices, returned by the `slice` method.
    type SliceIter<'a>: IntoIterator<Item = &'a [T]> + Default
    where
        T: 'a,
        Self: 'a;

    /// Iterator yielding mutable slices corresponding to a range of indices, returned by the `slice_mut` and `slice_mut_unchecked` methods.
    type SliceMutIter<'a>: IntoIterator<Item = &'a mut [T]> + Default
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

    /// Returns the index of the `element_ptr` pointing to an element of the vec.
    ///
    /// The complexity of this method depends on the particular `PinnedVec` implementation.
    /// However, making use of referential equality, it possible to perform much better than *O(n)*,
    /// where n is the vector length.
    ///
    /// For the two example implementations, complexity of this method:
    /// * *O(1)* for [FixedVec](https://crates.io/crates/orx-fixed-vec);
    /// * *O(f)* for [SplitVec](https://crates.io/crates/orx-split-vec) where f << n is the number of fragments.
    fn index_of_ptr(&self, element_ptr: *const T) -> Option<usize>;

    /// Appends an element to the back of a collection and returns a pointer to its position in the vector.
    fn push_get_ptr(&mut self, value: T) -> *const T;

    /// Creates an iterator of the pointers to the elements of the vec.
    ///
    /// # Safety
    ///
    /// The implementor guarantees that the pointers are valid and belong to the elements of the vector.
    /// However, the lifetime of the pointers might be extended by the caller;
    /// i.e., it is not bound to the lifetime of `&self`.
    ///
    /// Therefore, the caller is responsible for making sure that the obtained pointers are still
    /// valid before accessing through the pointers.
    unsafe fn iter_ptr<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i;

    /// Creates a reverse iterator of the pointers to the elements of the vec, starting from the last element to the first.
    ///
    /// # Safety
    ///
    /// The implementor guarantees that the pointers are valid and belong to the elements of the vector.
    /// However, the lifetime of the pointers might be extended by the caller;
    /// i.e., it is not bound to the lifetime of `&self`.
    ///
    /// Therefore, the caller is responsible for making sure that the obtained pointers are still
    /// valid before accessing through the pointers.
    unsafe fn iter_ptr_rev<'v, 'i>(&'v self) -> impl Iterator<Item = *const T> + 'i
    where
        T: 'i;

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

    /// Returns whether or not of the element with the given pointer belongs to this vector.
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
    fn contains_ptr(&self, element_ptr: *const T) -> bool;

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
    ///   none of them can have an invalid reference;
    /// * we cannot keep holding a reference to a vector element defined aliased the `clear` call,
    ///   since `clear` requires a `mut` reference.
    fn clear(&mut self);

    /// Returns the total number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;

    /// Provides detailed information of capacity state of the pinned vector.
    ///
    /// This information contains the current capacity which can be obtained by [`PinnedVec::capacity()`] method and extends with additional useful information.
    fn capacity_state(&self) -> CapacityState;

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

    /// Returns a reversed back-to-front iterator to elements of the vector.
    fn iter_rev(&self) -> Self::IterRev<'_>;
    /// Returns a reversed back-to-front iterator mutable references to elements of the vector.
    fn iter_mut_rev(&mut self) -> Self::IterMutRev<'_>;

    /// Returns the view on the required `range` as an iterator of slices:
    ///
    /// * returns an empty iterator if the range is out of bounds;
    /// * returns an iterator yielding ordered slices that forms the required range when chained.
    fn slices<R: RangeBounds<usize>>(&self, range: R) -> Self::SliceIter<'_>;

    /// Returns a mutable view on the required `range` as an iterator of mutable slices:
    ///
    /// * returns an empty iterator if the range is out of bounds;
    /// * returns an iterator yielding ordered slices that forms the required range when chained.
    fn slices_mut<R: RangeBounds<usize>>(&mut self, range: R) -> Self::SliceMutIter<'_>;

    /// Returns a pointer to the `index`-th element of the vector.
    ///
    /// Returns `None` if `index`-th position does not belong to the vector; i.e., if `index` is out of `capacity`.
    fn get_ptr(&self, index: usize) -> Option<*const T>;

    /// Returns a mutable pointer to the `index`-th element of the vector.
    ///
    /// Returns `None` if `index`-th position does not belong to the vector; i.e., if `index` is out of `capacity`.
    fn get_ptr_mut(&mut self, index: usize) -> Option<*mut T>;

    /// Forces the length of the vector to `new_len`.
    ///
    /// This is a low-level operation that maintains none of the normal invariants of the type.
    ///
    /// # Safety
    ///
    /// - `new_len` must be less than or equal to `capacity()`.
    /// - The elements at `old_len..new_len` must be initialized.
    unsafe fn set_len(&mut self, new_len: usize);

    /// Binary searches vector slice with a comparator function.
    ///
    /// The comparator function `f` should return an order code that indicates whether its argument is Less, Equal or Greater the desired target.
    /// If the vector is not sorted or if the comparator function does not implement an order consistent with the sort order of the underlying slice, the returned result is unspecified and meaningless.
    ///
    /// If the value is found then Result::Ok is returned, containing the index of the matching element.
    /// If there are multiple matches, then any one of the matches could be returned.
    ///
    /// If the value is not found then Result::Err is returned, containing the index where a matching element could be inserted while maintaining sorted order.
    ///
    /// See also binary_search and binary_search_by_key.
    ///
    /// # Examples
    ///
    /// Below example is taken from std::Vec since expected behavior of `PinnedVec` is exactly the same.
    ///
    /// Looks up a series of four elements.
    /// The first is found, with a uniquely determined position; the second and third are not found; the fourth could match any position in [1, 4].
    ///
    /// ```rust
    /// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    ///
    /// let seek = 13;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    /// let seek = 4;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    /// let seek = 100;
    /// assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    /// let seek = 1;
    /// let r = s.binary_search_by(|probe| probe.cmp(&seek));
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    fn binary_search_by<F>(&self, f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> Ordering;

    /// Binary searches this vector for the `search_value`.
    /// If the vector is not sorted, the returned result is unspecified and
    /// meaningless.
    ///
    /// If the value is found then [`Result::Ok`] is returned, containing the
    /// index of the matching element. If there are multiple matches, then any
    /// one of the matches could be returned
    ///
    /// If the value is not found then [`Result::Err`] is returned, containing
    /// the index where a matching element could be inserted while maintaining
    /// sorted order.
    ///
    /// # Examples
    ///
    /// Below examples are taken from std::Vec since expected behavior of `PinnedVec` is exactly the same.
    ///
    /// Looks up a series of four elements. The first is found, with a
    /// uniquely determined position; the second and third are not
    /// found; the fourth could match any position in `[1, 4]`.
    ///
    /// ```rust
    /// let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    ///
    /// assert_eq!(s.binary_search(&13),  Ok(9));
    /// assert_eq!(s.binary_search(&4),   Err(7));
    /// assert_eq!(s.binary_search(&100), Err(13));
    /// let r = s.binary_search(&1);
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    fn binary_search(&self, search_value: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        self.binary_search_by(|p| p.cmp(search_value))
    }

    /// Binary searches this vector with a key extraction function.
    ///
    /// Assumes that the vector is sorted by the key, for instance with
    /// `sort_by_key` using the same key extraction function.
    /// If the vector is not sorted by the key, the returned result is
    /// unspecified and meaningless.
    ///
    /// If the value is found then [`Result::Ok`] is returned, containing the
    /// index of the matching element. If there are multiple matches, then any
    /// one of the matches could be returned.
    ///
    /// If the value is not found then [`Result::Err`] is returned, containing
    /// the index where a matching element could be inserted while maintaining
    /// sorted order.
    ///
    /// # Examples
    ///
    /// Below examples are taken from std::Vec since expected behavior of `PinnedVec` is exactly the same.
    ///
    /// Looks up a series of four elements in a slice of pairs sorted by
    /// their second elements. The first is found, with a uniquely
    /// determined position; the second and third are not found; the
    /// fourth could match any position in `[1, 4]`.
    ///
    /// ```
    /// let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
    ///          (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
    ///          (1, 21), (2, 34), (4, 55)];
    ///
    /// assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
    /// assert_eq!(s.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
    /// assert_eq!(s.binary_search_by_key(&100, |&(a, b)| b), Err(13));
    /// let r = s.binary_search_by_key(&1, |&(a, b)| b);
    /// assert!(match r { Ok(1..=4) => true, _ => false, });
    /// ```
    fn binary_search_by_key<B, F>(&self, b: &B, mut f: F) -> Result<usize, usize>
    where
        F: FnMut(&T) -> B,
        B: Ord,
    {
        self.binary_search_by(|k| f(k).cmp(b))
    }

    /// Sorts the vector.
    ///
    /// This sort is stable.
    fn sort(&mut self)
    where
        T: Ord;

    /// Sorts the slice with a comparator function.
    ///
    /// This sort is stable.
    ///
    /// The comparator function must define a total ordering for the elements in the slice. If
    /// the ordering is not total, the order of the elements is unspecified. An order is a
    /// total order if it is (for all `a`, `b` and `c`):
    ///
    /// * total and antisymmetric: exactly one of `a < b`, `a == b` or `a > b` is true, and
    /// * transitive, `a < b` and `b < c` implies `a < c`. The same must hold for both `==` and `>`.
    ///
    /// For example, while [`f64`] doesn't implement [`Ord`] because `NaN != NaN`, we can use
    /// `partial_cmp` as our sort function when we know the slice doesn't contain a `NaN`.
    fn sort_by<F>(&mut self, compare: F)
    where
        F: FnMut(&T, &T) -> Ordering;

    /// Sorts the slice with a key extraction function.
    ///
    /// This sort is stable.
    fn sort_by_key<K, F>(&mut self, f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord;
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
