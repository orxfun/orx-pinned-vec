/// Provides detailed information of capacity state of the pinned vector.
///
/// This information contains the current capacity which can be obtained by `capacity()` method and extends with additional useful information.
///
/// * `FixedCapacity` variant only provides the current capacity.
/// However, its additional tag informs that this capacity is a hard constraint and the vector cannot grow beyond it.
/// * `DynamicCapacity` variant informs that the vector is capable of allocating and growing its capacity.
/// It provides `current_capacity` representing the current internal state of the vector.
/// Additionally, `maximum_concurrent_capacity` is provided.
/// This number represents the maximum number of elements that can safely be pushed to the vector in a concurrent program.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CapacityState {
    /// `FixedCapacity` variant only provides the current capacity.
    /// However, its additional tag informs that this capacity is a hard constraint and the vector cannot grow beyond it.
    FixedCapacity(usize),
    /// `DynamicCapacity` variant informs that the vector is capable of allocating and growing its capacity.
    /// It provides `current_capacity` representing the current internal state of the vector.
    /// Additionally, `maximum_concurrent_capacity` is provided.
    /// This number represents the maximum number of elements that can safely be pushed to the vector in a concurrent program.
    /// This value is often related with the capacity of the container holding meta information about allocations.
    /// Note that the dynamic vector can naturally grow beyond this number, this bound is only relevant when the vector is `Sync`ed among threads.
    DynamicCapacity {
        /// Capacity of current allocations owned by the vector.
        current_capacity: usize,
        /// Maximum capacity that can safely be reached by the vector in a concurrent program.
        /// This value is often related with the capacity of the container holding meta information about allocations.
        /// Note that the dynamic vector can naturally grow beyond this number, this bound is only relevant when the vector is `Sync`ed among threads.
        maximum_concurrent_capacity: usize,
    },
}

impl CapacityState {
    /// Capacity of current allocations owned by the vector.
    pub fn current_capacity(&self) -> usize {
        match self {
            Self::FixedCapacity(x) => *x,
            Self::DynamicCapacity {
                current_capacity,
                maximum_concurrent_capacity: _,
            } => *current_capacity,
        }
    }

    /// Maximum capacity that can safely be reached by the vector in a concurrent program.
    /// This value is often related with the capacity of the container holding meta information about allocations.
    /// Note that the dynamic vector can naturally grow beyond this number, this bound is only relevant when the vector is `Sync`ed among threads.
    pub fn maximum_concurrent_capacity(&self) -> usize {
        match self {
            Self::FixedCapacity(x) => *x,
            Self::DynamicCapacity {
                current_capacity: _,
                maximum_concurrent_capacity,
            } => *maximum_concurrent_capacity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_capacity() {
        assert_eq!(42, CapacityState::FixedCapacity(42).current_capacity());
        assert_eq!(
            7,
            CapacityState::DynamicCapacity {
                current_capacity: 7,
                maximum_concurrent_capacity: 42
            }
            .current_capacity()
        );
    }

    #[test]
    fn maximum_concurrent_capacity() {
        assert_eq!(
            42,
            CapacityState::FixedCapacity(42).maximum_concurrent_capacity()
        );
        assert_eq!(
            42,
            CapacityState::DynamicCapacity {
                current_capacity: 7,
                maximum_concurrent_capacity: 42
            }
            .maximum_concurrent_capacity()
        );
    }
}
