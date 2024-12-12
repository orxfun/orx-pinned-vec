use super::refmap::RefMap;
use crate::PinnedVec;

/// Tests the pinned vector guarantee on extending the vector;
/// panics if the pinned vector implementation `P` does not satisfy the required condition.
///
/// Tested pinned element guarantee:
///
/// * **G1: pinned elements on growth at the end**. This is one of the critical guarantees that the pinned vectors provide. We are adding **m** ≥ 1 elements to the end of the vector, and hence the vector reaches a length of **n + m**. Pinned vector guarantees that the memory locations of the **n** elements will not change due to this mutation.
///   * *Some such example mutation methods are **push**, **extend** or **extend_from_slice**.*
///   * *However, **insert** method is not covered since it is not an addition to the end of the vector.*
///   * *Notice that the standard vector does not satisfy this requirement.*
///   * *For many special data structures, such as concurrent collections or self referential collections, this is the necessary and sufficient pinned element guarantee.*
///
/// # Panics
///
/// Panics if the pinned vector implementation `P` does not satisfy the abovementioned pinned elements guarantee.
pub fn push<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    let mut refmap = RefMap::new(200, max_allowed_test_len);

    for i in 0..max_allowed_test_len {
        vec.push(i);
        refmap.set_reference(&vec, i);
        refmap.validate_references(&vec);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_push_empty() {
        let pinned_vec = TestVec::new(0);
        push(pinned_vec, 0);
    }

    #[test]
    fn test_push_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        push(pinned_vec, capacity);
    }

    #[test]
    fn test_push_medium() {
        let capacity = 512;
        let pinned_vec = TestVec::new(capacity);
        push(pinned_vec, capacity);
    }
}
