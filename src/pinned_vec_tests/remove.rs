use super::refmap::RefMap;
use crate::PinnedVec;

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
