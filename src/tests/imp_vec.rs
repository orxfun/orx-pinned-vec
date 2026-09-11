use crate::pinned_vec_tests::testvec::FixedCapVec;
use crate::*;

#[test]
fn imp_push_with_shared_reference_keeps_existing_pointers_valid() {
    let mut vec = FixedCapVec::new(4);
    vec.push(1);
    vec.push(2);

    let imp = vec.as_imp_vec();
    let first = &imp[0];

    imp.imp_push(3);
    imp.imp_push(4);

    assert_eq!(imp.len(), 4);
    assert_eq!(*first, 1);
    assert_eq!(imp[0], 1);
    assert_eq!(imp[1], 2);
    assert_eq!(imp[2], 3);
    assert_eq!(imp[3], 4);
}

#[test]
fn imp_push_get_ref_returns_reference_to_new_element() {
    let mut vec = FixedCapVec::new(4);
    vec.push(10);

    let imp = vec.as_imp_vec();
    let value = imp.imp_push_get_ref(20);

    assert_eq!(imp.len(), 2);
    assert_eq!(*value, 20);
    assert_eq!(imp[1], 20);
}

#[test]
fn imp_extend_from_slice_adds_elements_without_invalidating_existing_references() {
    let mut vec = FixedCapVec::new(8);
    vec.push(1);
    vec.push(2);

    let imp = vec.as_imp_vec();
    let first = &imp[0];

    imp.imp_extend_from_slice(&[3, 4, 5]);

    assert_eq!(imp.len(), 5);
    assert_eq!(*first, 1);
    assert_eq!(imp[0], 1);
    assert_eq!(imp[1], 2);
    assert_eq!(imp[2], 3);
    assert_eq!(imp[3], 4);
    assert_eq!(imp[4], 5);
}

#[test]
fn into_imp_vec_moves_vector_into_imp_vec() {
    let mut vec = FixedCapVec::new(4);
    vec.push(7);
    vec.push(8);

    let imp = vec.into_imp_vec();
    imp.imp_push(9);

    assert_eq!(imp.len(), 3);
    assert_eq!(imp[0], 7);
    assert_eq!(imp[1], 8);
    assert_eq!(imp[2], 9);
}
