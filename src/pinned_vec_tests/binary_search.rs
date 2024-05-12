use crate::PinnedVec;

pub fn binary_search<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    for i in 0..max_allowed_test_len {
        vec.push(i);
    }

    let n = vec.len();

    if n >= 42 {
        assert_eq!(vec.binary_search(&42), Ok(42));
        assert_eq!(vec.binary_search(&(n - 1)), Ok(n - 1));
    }
    assert_eq!(vec.binary_search(&n), Err(n));

    if n >= 42 {
        assert_eq!(vec.binary_search_by_key(&84, |x| x * 2), Ok(42));
        assert_eq!(vec.binary_search_by_key(&(2 * n - 2), |x| x * 2), Ok(n - 1));
    }
    assert_eq!(vec.binary_search_by_key(&(2 * n), |x| x * 2), Err(n));

    if n >= 42 {
        assert_eq!(vec.binary_search_by(|x| x.cmp(&42)), Ok(42));
        assert_eq!(vec.binary_search_by(|x| x.cmp(&(n - 1))), Ok(n - 1));
    }
    assert_eq!(vec.binary_search_by(|x| x.cmp(&n)), Err(n));

    vec
}
