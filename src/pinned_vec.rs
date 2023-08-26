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
    /// Returns the total number of elements the vector can hold without reallocating.
    fn capacity(&self) -> usize;
    /// Clears the vector, removing all values.
    ///
    /// Note that this method has no effect on the allocated capacity of the vector.
    fn clear(&mut self);
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

    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    fn insert(&mut self, index: usize, element: T);
    /// Returns true if the vector contains no elements.
    fn is_empty(&self) -> bool;
    /// Returns the number of elements in the vector, also referred to as its ‘length’.
    fn len(&self) -> usize;
    /// Removes the last element from a vector and returns it, or None if it is empty.
    fn pop(&mut self) -> Option<T>;
    /// Appends an element to the back of a collection.
    fn push(&mut self, value: T);
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    fn remove(&mut self, index: usize) -> T;

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
