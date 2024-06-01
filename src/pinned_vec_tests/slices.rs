use crate::PinnedVec;

pub fn slices<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let vec = slice(pinned_vec, max_allowed_test_len);
    slice_mut(vec, max_allowed_test_len)
}

fn slice<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    for i in 0..max_allowed_test_len {
        vec.push(i);
    }

    for i in (0..vec.len()).step_by(41) {
        let slice = vec.slices(0..i);
        let mut val = 0;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
        assert_eq!(i, val);
    }

    for i in (0..vec.len()).step_by(17) {
        let slice = vec.slices(i..max_allowed_test_len);
        let mut val = i;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
    }

    let ranges = [
        (10, max_allowed_test_len),
        (10, max_allowed_test_len / 2),
        (10, max_allowed_test_len / 4),
        (10, max_allowed_test_len / 8),
        (100, max_allowed_test_len),
        (100, max_allowed_test_len / 2),
        (100, max_allowed_test_len / 4),
        (100, max_allowed_test_len / 8),
    ];

    for (b, e) in ranges {
        let len = e.saturating_sub(b);
        let slice = vec.slices(b..e);
        let mut val = b;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
        assert_eq!(len, val - b);
    }

    vec
}

fn slice_mut<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    for i in 0..max_allowed_test_len {
        vec.push(i);
    }

    for i in (0..vec.len()).step_by(41) {
        let slice = vec.slices_mut(0..i);
        let mut val = 0;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
        assert_eq!(i, val);
    }

    for i in (0..vec.len()).step_by(17) {
        let slice = vec.slices_mut(i..max_allowed_test_len);
        let mut val = i;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
    }

    let ranges = [
        (10, max_allowed_test_len),
        (10, max_allowed_test_len / 2),
        (10, max_allowed_test_len / 4),
        (10, max_allowed_test_len / 8),
        (100, max_allowed_test_len),
        (100, max_allowed_test_len / 2),
        (100, max_allowed_test_len / 4),
        (100, max_allowed_test_len / 8),
    ];

    for (b, e) in ranges {
        let len = e.saturating_sub(b);
        let slice = vec.slices_mut(b..e);
        let mut val = b;
        for s in slice {
            for x in s {
                assert_eq!(*x, val);
                val += 1;
            }
        }
        assert_eq!(len, val - b);
    }

    let mut fill = |b: usize, e: usize| {
        let slice = vec.slices_mut(b..e);
        let mut val = b;
        for s in slice {
            for x in s {
                *x = 10 * val;
                val += 1;
            }
        }
    };
    fill(0, max_allowed_test_len);
    fill(0, 10);
    fill(10, 30);
    fill(30, 178);
    fill(178, 333);
    fill(333, 482);
    fill(333, max_allowed_test_len);

    for i in 0..max_allowed_test_len {
        assert_eq!(vec.get(i), Some(&(10 * i)));
    }

    vec
}
