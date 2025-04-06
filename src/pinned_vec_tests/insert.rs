use super::refmap::RefMap;
use crate::PinnedVec;

/// Tests the pinned vector guarantee on removing elements from arbitrary positions;
/// panics if the pinned vector implementation `P` does not satisfy the required condition.
///
/// Tested pinned element guarantee:
///
/// * **G3: pinned prior elements on insertions to arbitrary position**. Assume we are adding **m** ≥ 1 elements; however, not necessarily to the end of the vector this time. The earliest position of the inserted elements is **p** < (n-1). In this case, pinned vector guarantees that memory locations of the elements at positions 0..(p-1) will remain intact.
///   * *The example method is the **insert** method.*
///
/// # Panics
///
/// Panics if the pinned vector implementation `P` does not satisfy the abovementioned pinned elements guarantee.
pub fn insert<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    // insert at the end

    let first_half = max_allowed_test_len / 2;

    let mut refmap = RefMap::new(200, first_half);

    for i in 0..first_half {
        vec.push(i);
        refmap.set_reference(&vec, i);
        refmap.validate_references(&vec);
    }

    for i in first_half..max_allowed_test_len {
        vec.insert(first_half, i);
        refmap.validate_references(&vec);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_insert_empty() {
        let pinned_vec = TestVec::new(0);
        insert(pinned_vec, 0);
    }

    #[test]
    fn test_insert_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        insert(pinned_vec, capacity);
    }

    #[test]
    #[cfg(not(miri))]
    fn test_insert_medium() {
        let capacity = 256;
        let pinned_vec = TestVec::new(capacity);
        insert(pinned_vec, capacity);
    }
}
