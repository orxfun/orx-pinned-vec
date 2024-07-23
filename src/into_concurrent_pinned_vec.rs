use crate::{ConcurrentPinnedVec, PinnedVec};

/// A pinned vector which can be wrapped into a concurrent pinned vector.
pub trait IntoConcurrentPinnedVec<T>: PinnedVec<T> {
    /// Type of the concurrent pinned vector wrapper.
    type ConPinnedVec: ConcurrentPinnedVec<T, P = Self>;

    /// Converts the pinned vector into its concurrent wrapper.
    fn into_concurrent(self) -> Self::ConPinnedVec;
}
