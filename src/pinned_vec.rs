use std::fmt::{Debug, Formatter, Result};

/// A vector of elements of T which differs from the `std::vec::Vec` by the following feature:
///
/// * memory location of an element already pushed to the collection never changes and remains valid unless:
///     * the vector is `drop`ped,
///     * the vector is `clear`ed, `resize`d or `truncate`d,
///     * any element is `insert`ed into the vector,
///     * any element is `remove`d or `pop`ped from the vector.
///
/// Or, more briefly, `push`ing or `extend`ing the vector does not change memory locations
/// of already added elements, and hence, the corresponding references remain valid.
///
/// # Safety
///
/// This trait can be considered as a marker trait:
/// its methods are relevant for the useful 'vec'-related side of the trait,
/// rather than the pinned side.
/// The implementing struct must guarantee that pushing or extending the vector
/// does not cause the memory locations of already added elements to change.
pub trait PinnedVec<T> {
    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    ///
    /// # Safety
    ///
    /// `clear` operation is **safe** both when `T: NotSelfReferencingVecItem` or `T: SelfReferencingVecItem`.
    ///
    /// The prior is obvious; the reason why `T: SelfReferencingVecItem` is safe is as follows:
    ///
    /// * elements holding references to each other will be cleaned all together; hence,
    /// none of them can have an invalid reference;
    /// * we cannot have a reference to a vector element defined before the `clear`,
    /// since `clear` requires a `mut` reference.
    fn clear(&mut self);
    /// Returns the total number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;
    /// Clones and appends all elements in a slice to the Vec.
    ///
    /// Iterates over the slice other, clones each element, and then appends it to this Vec. The other slice is traversed in-order.
    ///
    /// Note that this function is same as extend except that it is specialized to work with slices instead. If and when Rust gets specialization this function will likely be deprecated (but still available).
    fn extend_from_slice(&mut self, other: &[T])
    where
        T: Clone;

    /// Returns a reference to an element with the given `index`,
    /// returns None if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&T>;
    /// Returns a mutable reference to an element with the given `index`,
    /// returns None if the index is out of bounds.
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;
    /// Returns a reference to an element or subslice, without doing bounds checking.
    ///
    /// For a safe alternative see [`get`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked(&self, index: usize) -> &T;
    /// Returns a mutable reference to an element or subslice, without doing bounds checking.
    ///
    /// For a safe alternative see [`get_mut`].
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is *[undefined behavior]*
    /// even if the resulting reference is not used.
    unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T;

    /// Returns true if the vector contains no elements.
    fn is_empty(&self) -> bool;
    /// Returns the number of elements in the vector, also referred to as its ‘length’.
    fn len(&self) -> usize;
    /// Appends an element to the back of a collection.
    fn push(&mut self, value: T);

    // unsafe
    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `insert` is unsafe since insertion of a new element at an arbitrary position of the vector
    /// typically changes the positions of already existing elements.
    ///
    /// When the elements are holding references to other elements of the vector,
    /// this change in positions makes the references invalid.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
    unsafe fn unsafe_insert(&mut self, index: usize, element: T);
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `remove` is unsafe since removal of an element at an arbitrary position of the vector
    /// typically changes the positions of already existing elements.
    ///
    /// Further, it is possible that at least one of the remaining elements is
    /// pointing to the element which is being removed.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
    unsafe fn unsafe_remove(&mut self, index: usize) -> T;
    /// Removes the last element from a vector and returns it, or None if it is empty.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `pop` is unsafe since it is possible that at least one of the remaining elements is
    /// pointing to the last element which is being popped.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
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
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `swap` is unsafe since it is possible that other elements are referencing one of the
    /// elements to be swapped.
    /// The swap operation does not lead to an undefined behavior in the classical sense;
    /// however, would invalidate the inter-elements-references.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
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
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// `truncate` is unsafe since it is possible that remaining elements are referencing
    /// to elements which are dropped by the truncate method.
    ///
    /// On the other hand, any vector implementing `PinnedVec<T>` where `T: NotSelfRefVecItem`
    /// implements `PinnedVecSimple<T>` which implements the safe version of this method.
    unsafe fn unsafe_truncate(&mut self, len: usize);
    /// Creates and returns a clone of the vector.
    ///
    /// # Safety
    ///
    /// This operation is **unsafe** when `T` is not `NotSelfRefVecItem`.
    /// To pick the conservative approach, every T which does not implement `NotSelfRefVecItem`
    /// is assumed to be a vector item referencing other vector items.
    ///
    /// To understand why `clone` is unsafe when `T` is not `NotSelfRefVecItem`,
    /// consider the following example.
    ///
    /// * let `vec` be the initial vector with self referencing vector elements.
    /// * let `cl` be the clone of `A`; i.e., 'let cl = vec.clone()`.
    /// * In this case, elements of `cl` are pointing to elements of `vec`.
    ///     * This is not correct, as these references are to be kept internal to the vector.
    ///     * Furthermore, if `vec` is dropped, `cl` elements will contain invalid references leading to UB.
    unsafe fn unsafe_clone(&self) -> Self
    where
        T: Clone;

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
