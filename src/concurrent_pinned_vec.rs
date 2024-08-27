use crate::{PinnedVec, PinnedVecGrowthError};
use std::ops::{Range, RangeBounds};

/// A wrapper for a pinned vector which provides additional guarantees for concurrent programs.
///
/// Note that a concurrent pinned vec inherits pinned memory location guarantees.
///
/// The struct encapsulates many methods of the pinned vector which are not suitable for concurrent programs.
/// Further, it exposes new and mostly unsafe methods for allowing performant concurrent collections.
/// It is designed to be a core structure for concurrent collections with a safe api.
pub trait ConcurrentPinnedVec<T> {
    /// Type of the wrapped pinned vector.
    type P: PinnedVec<T>;

    /// Converts back to the underlying pinned vector with the given length.
    ///
    /// # Safety
    ///
    /// This method is unsafe due to the following.
    /// The concurrent pinned vector is the core data structure for different concurrent collections
    /// which allow writing to the vector in different ways.
    /// The wrapper is responsible to deal with the gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn into_inner(self, len: usize) -> Self::P;

    /// Clones the concurrent pinned vector with for the first `len` elements.
    /// The created concurrent vector will have the same capacity and maximum capacity as this collection;
    /// however, only the values within 0..len will be cloned to the target.
    ///
    /// # Safety
    ///
    /// This method is unsafe due to the following.
    /// The concurrent pinned vector is the core data structure for different concurrent collections
    /// which allow writing to the vector in different ways.
    /// The wrapper is responsible to deal with the gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn clone_with_len(&self, len: usize) -> Self
    where
        T: Clone;

    // &self get

    /// Returns an iterator over positions `0..len` of the vector.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn iter<'a>(&'a self, len: usize) -> impl Iterator<Item = &'a T> + 'a
    where
        T: 'a;

    /// Returns a reference to the element at the `index`-th position.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if the entry at position `index` is written.
    unsafe fn get(&self, index: usize) -> Option<&T>;

    /// Returns a mutable reference to the element at the `index`-th position.
    ///
    /// # Safety
    ///
    /// This method is used to write to the vector.
    /// Therefore, its position will initially be uninitialized; hence, reading the pointer might result in UB.
    unsafe fn get_ptr_mut(&self, index: usize) -> *mut T;

    /// Returns an iterator of mutable slices to the elements extending over positions `range` of the vector.
    ///
    /// # Safety
    ///
    /// This method is used to write to the vector.
    /// Therefore, the positions will initially be uninitialized; hence, reading from the slices might result in UB.
    unsafe fn slices_mut<R: RangeBounds<usize>>(
        &self,
        range: R,
    ) -> <Self::P as PinnedVec<T>>::SliceMutIter<'_>;

    /// Returns an iterator of slices to the elements extending over positions `range` of the vector.
    fn slices<R: RangeBounds<usize>>(&self, range: R) -> <Self::P as PinnedVec<T>>::SliceIter<'_>;

    // capacity

    /// Returns the maximum possible capacity that the vector can concurrently grow to without requiring a `&mut self` reference.
    ///
    /// In order to grow beyond this capacity, `reserve_maximum_concurrent_capacity` method can be used.
    fn max_capacity(&self) -> usize;

    /// Returns the current capacity of the vector, which is actually allocated.
    fn capacity(&self) -> usize;

    /// Tries to concurrently grow the capacity of the vector to at least `new_capacity`. Returns:
    /// * Ok of the new capacity if succeeds
    /// * Err otherwise.
    ///
    /// Behavior of this method is deterministic.
    /// The method always succeeds (fails) if `new_capacity <= self.max_capacity()` (otherwise).
    ///
    /// If the method returns an error, `reserve_maximum_concurrent_capacity` method can be used; however, with a `&mut self` reference.
    /// Then, `grow_to` will succeed.
    fn grow_to(&self, new_capacity: usize) -> Result<usize, PinnedVecGrowthError>;

    /// Tries to concurrently grow the capacity of the vector to at least `new_capacity`. Returns:
    /// * Ok of the new capacity if succeeds
    /// * Err otherwise.
    ///
    /// Behavior of this method is deterministic.
    /// The method always succeeds (fails) if `new_capacity <= self.max_capacity()` (otherwise).
    ///
    /// If the method returns an error, `reserve_maximum_concurrent_capacity` method can be used;
    /// however, with a `&mut self` reference.
    /// Then, `grow_to` will succeed.
    ///
    /// During growth:
    ///
    /// * length of the vector is increased to its new capacity;
    /// * the elements in the range `len..capacity` are filled with the values
    /// obtained by repeatedly calling the function `fill_with`.
    fn grow_to_and_fill_with<F>(
        &self,
        new_capacity: usize,
        fill_with: F,
    ) -> Result<usize, PinnedVecGrowthError>
    where
        F: Fn() -> T;

    /// Fills the provided `range` with elements created by successively calling the `fill_with` function.
    fn fill_with<F>(&self, range: Range<usize>, fill_with: F)
    where
        F: Fn() -> T;

    /// Increases the `maximum_capacity` to the `new_maximum_capacity`.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    /// The vector must be gap-free while increasing the maximum capacity.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn reserve_maximum_concurrent_capacity(
        &mut self,
        len: usize,
        new_maximum_capacity: usize,
    ) -> usize;

    // &mut self

    /// Sets the length of the underlying pinned vector to the given `len`.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn set_pinned_vec_len(&mut self, len: usize);

    /// Returns a mutable iterator over positions `0..len` of the vector.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn iter_mut<'a>(&'a mut self, len: usize) -> impl Iterator<Item = &'a mut T> + 'a
    where
        T: 'a;

    /// Returns a reference to the element at the `index`-th position.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if the entry at position `index` is written.
    unsafe fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Clears the concurrent pinned vector.
    ///
    /// # Safety
    ///
    /// This method is unsafe since the concurrent pinned vector might contain gaps.
    ///
    /// This method can safely be called if entries in all positions `0..len` are written.
    unsafe fn clear(&mut self, len: usize);
}
