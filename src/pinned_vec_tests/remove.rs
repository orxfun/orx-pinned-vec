use super::refmap::RefMap;
use crate::PinnedVec;

/// Tests the pinned vector guarantee on removing elements from arbitrary positions;
/// panics if the pinned vector implementation `P` does not satisfy the required condition.
///
/// Tested pinned element guarantee:
///
/// * **G4: pinned prior elements in removals from arbitrary position**. Lastly, assume that we are removing **m** ∈ [1, n] elements from the arbitrary positions of the vector leading to a final vector length of **n - m**. Let **p** be the earliest position of the removed elements. Pinned vector then guarantees that memory locations of the elements at positions 0..(p-1) will remain intact.
///   * *The example method is the **remove** method.*
///
/// # Panics
///
/// Panics if the pinned vector implementation `P` does not satisfy the abovementioned pinned elements guarantee.
pub fn remove<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    let first_half = max_allowed_test_len / 2;

    let mut refmap = RefMap::new(200, first_half);

    for i in 0..first_half {
        vec.push(i);
        refmap.set_reference(&vec, i);
    }
    for i in first_half..max_allowed_test_len {
        vec.push(i);
    }

    for i in first_half..max_allowed_test_len {
        let removed = vec.remove(first_half);
        assert_eq!(i, removed);
        refmap.validate_references(&vec);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_remove_empty() {
        let pinned_vec = TestVec::new(0);
        remove(pinned_vec, 0);
    }

    #[test]
    fn test_remove_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        remove(pinned_vec, capacity);
    }

    #[test]
    fn test_remove_medium() {
        let capacity = 256;
        let pinned_vec = TestVec::new(capacity);
        remove(pinned_vec, capacity);
    }
}
