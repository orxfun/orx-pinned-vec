use crate::PinnedVec;

pub fn grow<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;

    // empty - no growth
    vec.clear();
    let cap = vec.capacity();
    let new_cap = vec.grow_and_initialize(0, || 444444).expect("must succeed");
    assert_eq!(new_cap, cap);
    assert_eq!(vec.len(), cap);
    assert_eq!(vec.capacity(), cap);

    // empty - growth
    vec.clear();
    let cap = vec.capacity();
    let req_cap = 56.min(max_allowed_test_len);
    let new_cap = vec
        .grow_and_initialize(req_cap, || 444444)
        .expect("must succeed");
    assert!(new_cap >= req_cap);
    assert!(new_cap >= cap);
    assert_eq!(vec.capacity(), new_cap);
    assert_eq!(vec.len(), new_cap);
    for i in 0..cap {
        assert_eq!(vec.get(i), Some(&444444));
    }

    // half full - no growth
    vec.clear();
    for i in 0..22.min(max_allowed_test_len) {
        vec.push(i);
    }
    let (len, cap) = (vec.len(), vec.capacity());
    let new_cap = vec.grow_and_initialize(0, || 444444).expect("must succeed");
    assert_eq!(new_cap, cap);
    assert_eq!(vec.len(), cap);
    assert_eq!(vec.capacity(), cap);
    for i in 0..len {
        assert_eq!(vec.get(i), Some(&i));
    }
    for i in len..cap {
        assert_eq!(vec.get(i), Some(&444444));
    }

    // half full - growth
    vec.clear();
    for i in 0..22.min(max_allowed_test_len) {
        vec.push(i);
    }
    let (len, cap) = (vec.len(), vec.capacity());
    let req_cap = 56.min(max_allowed_test_len);
    let new_cap = vec
        .grow_and_initialize(req_cap, || 444444)
        .expect("must succeed");
    assert!(new_cap >= req_cap);
    assert!(new_cap >= cap);
    assert_eq!(vec.capacity(), new_cap);
    assert_eq!(vec.len(), new_cap);
    for i in 0..len {
        assert_eq!(vec.get(i), Some(&i));
    }
    for i in len..cap {
        assert_eq!(vec.get(i), Some(&444444));
    }

    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn test_pop_empty() {
        let pinned_vec = TestVec::new(0);
        grow(pinned_vec, 0);
    }

    #[test]
    fn test_pop_small() {
        let capacity = 40;
        let pinned_vec = TestVec::new(capacity);
        grow(pinned_vec, capacity);
    }

    #[test]
    fn test_pop_medium() {
        let capacity = 1024 * 64;
        let pinned_vec = TestVec::new(capacity);
        grow(pinned_vec, capacity);
    }

    #[test]
    fn test_pop_large() {
        let capacity = 1024 * 1024;
        let pinned_vec = TestVec::new(capacity);
        grow(pinned_vec, capacity);
    }
}
