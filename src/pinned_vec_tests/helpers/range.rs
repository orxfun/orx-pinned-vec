use std::ops::RangeBounds;

pub(crate) fn range_start<R: RangeBounds<usize>>(range: &R) -> usize {
    match range.start_bound() {
        std::ops::Bound::Excluded(x) => x + 1,
        std::ops::Bound::Included(x) => *x,
        std::ops::Bound::Unbounded => 0,
    }
}
pub(crate) fn range_end<R: RangeBounds<usize>>(range: &R, vec_len: usize) -> usize {
    match range.end_bound() {
        std::ops::Bound::Excluded(x) => *x,
        std::ops::Bound::Included(x) => x + 1,
        std::ops::Bound::Unbounded => vec_len,
    }
}
