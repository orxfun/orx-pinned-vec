use crate::{NotSelfRefVecItem, PinnedVec};

/// `PinnedVecSimple` is a `PinnedVec` where the elements satisfy the trait bound
/// `T: NotSelfRefVecItem.
///
/// In other words, elements of the vector does not hold references to other
/// elements of the same vector.
/// Note that this is satisfied for all `std::vec::Vec`s.
///
/// On the other hand, `PinnedVec` is designed to conveniently build more complex
/// data structures while holding children of these data structures in a vec-like
/// layout for better cache locality and to reduce heap allocations.
///
/// These structures often contain child structures referencing each other;
/// such as the `parent` or `children` relations of a tree.
/// `PinnedVec` aims at guaranteeing that these internal references are kept valid.
/// This makes methods `insert`, `remove` and `pop` unsafe.
///
/// Since the aforementioned safety concern is absent when elements do not hold such internal references;
/// i.e., when `T: NotSelfRefVecItem`,
/// such vectors automatically implement `PinnedVecSimple` which enables safe calls
/// to these methods.
///
/// # Safety
///
/// Picking the more conservative and safer approach;
/// the default versions of methods `insert`, `remove` and `pop` are unsafe.
///
/// In order to be able to make safe calls to these methods,
/// once must explicitly implement `NotSelfRefVecItem` for the element type.
/// This is a marker trait, and hence, easy to implement.
///
/// For convenience,
/// this crate already contains implementations for the primitive structs
/// such as numbers, string or bool.
pub trait PinnedVecSimple<T>: PinnedVec<T>
where
    T: NotSelfRefVecItem,
{
    /// Inserts an element at position index within the vector, shifting all elements after it to the right.
    ///
    /// # Panics
    /// Panics if `index >= len`.
    ///
    /// # Safety
    ///
    /// `insert` operation for a `PinnedVecSimple` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    fn insert(&mut self, index: usize, element: T);
    /// Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    ///
    /// # Panics
    ///
    /// Panics if index is out of bounds.
    ///
    /// # Safety
    ///
    /// `remove` operation for a `PinnedVecSimple` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    fn remove(&mut self, index: usize) -> T;
    /// Removes the last element from a vector and returns it, or None if it is empty.
    ///
    /// # Safety
    ///
    /// `pop` operation for a `PinnedVecSimple` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    fn pop(&mut self) -> Option<T>;
    /// Creates and returns a clone of the vector.
    ///
    /// # Safety
    ///
    /// `clone` operation for a `PinnedVecSimple` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    fn clone(&self) -> Self
    where
        T: Clone;
}

impl<T, V> PinnedVecSimple<T> for V
where
    T: NotSelfRefVecItem,
    V: PinnedVec<T>,
{
    #[inline(always)]
    fn insert(&mut self, index: usize, element: T) {
        unsafe { self.unsafe_insert(index, element) }
    }
    #[inline(always)]
    fn remove(&mut self, index: usize) -> T {
        unsafe { self.unsafe_remove(index) }
    }
    #[inline(always)]
    fn pop(&mut self) -> Option<T> {
        unsafe { self.unsafe_pop() }
    }
    #[inline(always)]
    fn clone(&self) -> Self
    where
        T: Clone,
    {
        unsafe { self.unsafe_clone() }
    }
}
