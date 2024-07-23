use super::refmap::RefMap;
use crate::PinnedVec;

pub fn insert<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

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
    fn test_insert_medium() {
        let capacity = 256;
        let pinned_vec = TestVec::new(capacity);
        insert(pinned_vec, capacity);
    }
}
