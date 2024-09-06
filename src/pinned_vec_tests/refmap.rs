use crate::PinnedVec;
use alloc::collections::btree_map::BTreeMap;
use core::ops::{Deref, DerefMut};

pub struct RefMap(BTreeMap<usize, Option<*const usize>>);

impl Deref for RefMap {
    type Target = BTreeMap<usize, Option<*const usize>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for RefMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl RefMap {
    pub fn new(max_num_indices: usize, max_len: usize) -> Self {
        fn random_idx(i: usize, max_len: usize) -> usize {
            let x = (((((2 * i + 7) / 3) + max_len) * 5).saturating_sub(71)) + 44;
            x % max_len
        }

        let mut map = BTreeMap::new();
        if max_len > 0 {
            for i in 0..max_num_indices {
                let idx = random_idx(i, max_len);
                map.entry(idx).or_insert(None);
            }
        }
        Self(map)
    }

    pub fn set_reference<P: PinnedVec<usize>>(&mut self, pinned_vec: &P, i: usize) {
        if let Some(reference) = self.get_mut(&i) {
            let element = pinned_vec.get(i).expect("entry exists");
            let addr = element as *const usize;
            *reference = Some(addr);
        }
    }

    pub fn drop_reference(&mut self, i: usize) {
        if let Some(reference) = self.get_mut(&i) {
            *reference = None;
        }
    }

    pub fn validate_references<P: PinnedVec<usize>>(&self, pinned_vec: &P) {
        for (i, addr) in &self.0 {
            if let Some(addr) = addr {
                let element = pinned_vec.get(*i).expect("must be some");
                assert_eq!(i, element);

                let element_addr = element as *const usize;
                assert_eq!(
                    *addr, element_addr,
                    "element address has changed while growing"
                );
                let value_at_addr = unsafe { core::ptr::read(*addr) };
                assert_eq!(*i, value_at_addr, "value at address has changed");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pinned_vec_tests::testvec::TestVec;

    #[test]
    fn deref() {
        let max_num_indices = 10;
        let max_len = 20;
        let refmap = RefMap::new(max_num_indices, max_len);
        assert_eq!(refmap.deref(), &refmap.0);
    }

    #[test]
    fn deref_mut() {
        let max_num_indices = 10;
        let max_len = 1;
        let mut refmap1 = RefMap::new(max_num_indices, max_len);
        let mut refmap2 = RefMap::new(max_num_indices, max_len);

        refmap1.remove(&0);
        refmap2.0.remove(&0);

        assert_eq!(&refmap1.0, &refmap2.0);
    }

    #[test]
    fn new() {
        let max_num_indices = 10;
        let max_len = 20;
        let refmap = RefMap::new(max_num_indices, max_len);
        assert!(refmap.0.len() <= max_num_indices);
        assert!(refmap.0.keys().all(|x| x < &max_len));
    }

    #[test]
    fn set_reference() {
        let mut pinned_vec = TestVec::new(10);
        pinned_vec.push(10);

        let max_num_indices = 10;
        let max_len = 1;
        let mut refmap = RefMap::new(max_num_indices, max_len);

        assert!(refmap.get(&0).expect("is some").is_none());

        refmap.set_reference(&pinned_vec, 0);

        assert_eq!(
            refmap.get(&0).expect("is-some"),
            &Some(pinned_vec.get(0).expect("is-some") as *const usize)
        );
    }

    #[test]
    fn drop_reference() {
        let mut pinned_vec = TestVec::new(10);
        pinned_vec.push(10);
        pinned_vec.push(20);

        let max_num_indices = 10;
        let max_len = 1;
        let mut refmap = RefMap::new(max_num_indices, max_len);

        refmap.set_reference(&pinned_vec, 0);
        assert_eq!(
            refmap.get(&0).expect("is-some"),
            &Some(pinned_vec.get(0).expect("is-some") as *const usize)
        );

        refmap.drop_reference(1);
        assert_eq!(
            refmap.get(&0).expect("is-some"),
            &Some(pinned_vec.get(0).expect("is-some") as *const usize)
        );

        refmap.drop_reference(0);
        assert!(refmap.get(&0).expect("is some").is_none());
    }
}
