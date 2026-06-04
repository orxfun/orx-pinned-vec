use crate::utils::slice::index_of_ptr;

#[test]
fn index_of_ptr_empty_slice() {
    let slice: &[u8] = &[];
    let ptr_to_check = slice.as_ptr();
    let idx = index_of_ptr(slice, ptr_to_check);
    assert_eq!(idx, None);
}

#[test]
fn index_of_ptr_out_of_bounds() {
    let vec = [0, 1, 2, 3, 4, 5, 6, 7];

    let ptr_to_check = unsafe { vec.as_ptr().add(4) };
    assert_eq!(unsafe { *ptr_to_check }, 4);

    let idx = index_of_ptr(&vec[0..3], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec[0..4], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec[5..], ptr_to_check);
    assert_eq!(idx, None);
}

#[test]
fn index_of_ptr_in_bounds() {
    let vec = [0, 1, 2, 3, 4, 5];

    let ptr_to_check = unsafe { vec.as_ptr().add(3) };
    assert_eq!(unsafe { *ptr_to_check }, 3);

    let idx = index_of_ptr(&vec[..], ptr_to_check);
    assert_eq!(idx, Some(3));

    let idx = index_of_ptr(&vec[0..4], ptr_to_check);
    assert_eq!(idx, Some(3));

    let idx = index_of_ptr(&vec[0..5], ptr_to_check);
    assert_eq!(idx, Some(3));

    let idx = index_of_ptr(&vec[1..], ptr_to_check);
    assert_eq!(idx, Some(2));

    let idx = index_of_ptr(&vec[2..], ptr_to_check);
    assert_eq!(idx, Some(1));

    let idx = index_of_ptr(&vec[3..], ptr_to_check);
    assert_eq!(idx, Some(0));
}

#[test]
fn index_of_ptr_different_allocation() {
    let vec1 = [0, 1, 2, 3, 4, 5];
    let ptr_to_check = unsafe { vec1.as_ptr().add(3) };
    assert_eq!(unsafe { *ptr_to_check }, 3);

    let vec2 = [0, 1, 2, 3, 4, 5];

    let idx = index_of_ptr(&vec2[..], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec2[0..4], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec2[0..5], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec2[1..], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec2[2..], ptr_to_check);
    assert_eq!(idx, None);

    let idx = index_of_ptr(&vec2[3..], ptr_to_check);
    assert_eq!(idx, None);
}
