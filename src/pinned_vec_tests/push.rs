use super::refmap::RefMap;
use crate::PinnedVec;

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
