use super::refmap::RefMap;
use crate::PinnedVec;

pub fn unsafe_writer<P: PinnedVec<usize>>(pinned_vec: P, max_allowed_test_len: usize) -> P {
    let mut vec = pinned_vec;
    vec.clear();

    let mut refmap = RefMap::new(200, max_allowed_test_len);

    let len1 = max_allowed_test_len / 2;

    for i in 0..len1 {
        vec.push(i);
        refmap.set_reference(&vec, i);
        refmap.validate_references(&vec);
    }

    assert_eq!(vec.len(), len1);

    for i in len1..max_allowed_test_len {
        if let Some(ptr) = unsafe { vec.get_ptr_mut(i) } {
            unsafe { *ptr = i };

            unsafe { vec.set_len(i + 1) };
            assert_eq!(vec.len(), i + 1);

            assert_eq!(vec.get(i), Some(&i));

            refmap.set_reference(&vec, i);
            refmap.validate_references(&vec);
        }
    }

    for i in vec.capacity()..(vec.capacity() + 100) {
        assert!(unsafe { vec.get_ptr_mut(i) }.is_none());
    }

    vec
}
