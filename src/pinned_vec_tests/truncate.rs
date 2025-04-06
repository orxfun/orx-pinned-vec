use super::refmap::RefMap;
use crate::PinnedVec;

/// Tests the pinned vector guarantee on removing elements from the end;
/// panics if the pinned vector implementation `P` does not satisfy the required condition.
///
/// Tested pinned element guarantee:
///
/// * **G2: pinned elements on removals from the end**. In this case, we are removing **m** ∈ [1, n] elements from the end of the vector leading to the final vector length of **n - m**. Pinned vector guarantees that memory locations of these remaining **n - m** elements do not change.
///   * *Some such example methods are **pop**, **truncate** or **clear**.*
pub fn truncate<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
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

    for _ in first_half..max_allowed_test_len {
        let new_len = vec.len() - 1;
        vec.truncate(new_len);
        refmap.validate_references(&vec);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_truncate_empty() {
        let pinned_vec = TestVec::new(0);
        truncate(pinned_vec, 0);
    }

    #[test]
    fn test_truncate_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        truncate(pinned_vec, capacity);
    }

    #[test]
    #[cfg(not(miri))]
    fn test_truncate_medium() {
        let capacity = 512;
        let pinned_vec = TestVec::new(capacity);
        truncate(pinned_vec, capacity);
    }
}
