/// Error occurred during an attempt to increase capacity of the pinned vector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PinnedVecGrowthError {
    /// An error stating that the vector is only allowed to grow if its entire current capacity is used.
    CanOnlyGrowWhenVecIsAtCapacity,
    /// An error which is observed when a pinned vector attempted to increase its capacity while keeping its already added elements pinned in their locations.
    FailedToGrowWhileKeepingElementsPinned,
}
