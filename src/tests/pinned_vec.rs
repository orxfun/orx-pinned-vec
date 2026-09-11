use crate::pinned_vec_tests::testvec::FixedCapVec;
use crate::*;

#[test]
fn is_empty() {
    let mut vec = FixedCapVec::new(5);
    assert!(vec.is_empty());

    vec.push(1);
    assert!(!vec.is_empty());

    vec.push(2);
    vec.push(3);
    assert!(!vec.is_empty());

    vec.clear();
    assert!(vec.is_empty());
}
