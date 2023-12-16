use crate::{NotSelfRefVecItem, PinnedVec};

/// `PinnedVecSimple` is a `PinnedVec` where the elements satisfy the trait bound
/// `T: NotSelfRefVecItem`.
///
/// We never manually implement this trait. Instead:
///
/// * if `V` implements `PinnedVec<T>` for some `T` such that
/// * `T` implements `NotSelfRefVecItem`
///
/// => then `V` auto-implements `PinnedVecSimple`.
///
/// Therefore, `T: NotSelfRefVecItem` is sufficient to inform that the elements of the vector do not hold references to each other.
/// This is critical as it enables the safe versions of the following methods which are otherwise unsafe:
///
/// * `insert`
/// * `remove`
/// * `pop`
/// * `swap`
/// * `truncate`
/// * `clone`
///
/// Note that having `NotSelfRefVecItem` elements is actually the usual in rust, as the opposite is challenging.
/// Safely and conveniently enabling such rust-difficult data structures (linked lists, trees, graphs, etc)
/// is exactly the goal of the `PinnedVec` trait and benefiting data structures such as the `orx_imp_vec::ImpVec`.
///
/// `NotSelfRefVecItem` is a marker trait which is implemented for most primitives; however, one needs to implement
/// for new types to explicitly state that the type is <ins>not</ins> self-referential.
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
    /// `swap` operation for a `PinnedVecSimple` where the elements are `T: NotSelfRefVecItem` is **safe**;
    /// in this case, pinned vector shares the same safety requirements as `std::vec::Vec` which is readily
    /// provided by the borrow checker.
    fn swap(&mut self, a: usize, b: usize);
    /// Shortens the vector, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater than the vector's current length, this has no
    /// effect.
    fn truncate(&mut self, len: usize);
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
    fn swap(&mut self, a: usize, b: usize) {
        unsafe { self.unsafe_swap(a, b) }
    }
    #[inline(always)]
    fn truncate(&mut self, len: usize) {
        unsafe { self.unsafe_truncate(len) }
    }
    #[inline(always)]
    fn clone(&self) -> Self
    where
        T: Clone,
    {
        unsafe { self.unsafe_clone() }
    }
}
