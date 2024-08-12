use crate::{ConcurrentPinnedVec, PinnedVec};

/// A pinned vector which can be wrapped into a concurrent pinned vector.
pub trait IntoConcurrentPinnedVec<T>: PinnedVec<T> {
    /// Type of the concurrent pinned vector wrapper.
    type ConPinnedVec: ConcurrentPinnedVec<T, P = Self>;

    /// Converts the pinned vector into its concurrent wrapper.
    fn into_concurrent(self) -> Self::ConPinnedVec;

    /// Converts the pinned vector into its concurrent wrapper.
    /// During conversion:
    ///
    /// * length of the vector is increased to its capacity;
    /// * the elements in the range `len..capacity` are filled with the values
    /// obtained by repeatedly calling the function `fill_with`.
    fn into_concurrent_filled_with<F>(self, fill_with: F) -> Self::ConPinnedVec
    where
        F: Fn() -> T;
}
