use super::refmap::RefMap;
use crate::PinnedVec;
use alloc::vec::Vec;

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
pub fn extend<P: PinnedVec<usize> + Sized>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    let mut refmap = RefMap::new(200, max_allowed_test_len);

    let average_extend_length = [1, max_allowed_test_len / 37]
        .into_iter()
        .max()
        .expect("cannot be None");
    let num_chunks = max_allowed_test_len / average_extend_length;
    let mut extend_lengths = Vec::new();
    for _ in 0..num_chunks {
        extend_lengths.push(average_extend_length);
    }
    let last_chunk_len = max_allowed_test_len - num_chunks * average_extend_length;
    extend_lengths.push(last_chunk_len);
    assert_eq!(extend_lengths.iter().sum::<usize>(), max_allowed_test_len);

    for slice_len in extend_lengths {
        let begin = vec.len();
        let slice: Vec<_> = (begin..(begin + slice_len)).collect();
        vec.extend_from_slice(&slice);

        for i in begin..(begin + slice_len) {
            refmap.set_reference(&vec, i);
        }

        refmap.validate_references(&vec);
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_extend_empty() {
        let pinned_vec = TestVec::new(0);
        extend(pinned_vec, 0);
    }

    #[test]
    fn test_extend_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        extend(pinned_vec, capacity);
    }

    #[test]
    fn test_extend_medium() {
        let capacity = 512;
        let pinned_vec = TestVec::new(capacity);
        extend(pinned_vec, capacity);
    }
}
