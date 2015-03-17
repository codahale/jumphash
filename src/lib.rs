//! An implementation of the [Jump Consistent Hash](http://arxiv.org/abs/1406.2294) algorithm by
//! Lamping & Veach.

#![feature(core,hash)]

use std::collections::BTreeSet;
use std::hash::{Hash, Hasher, SipHasher};

/// Returns the bucket for the given key and set of buckets.
///
/// # Panics
//
/// * `select` will panic if the set of buckets is empty.
pub fn select<'a, T: Hash, E: Ord>(key: &T, buckets: &'a BTreeSet<E>) -> &'a E {
    assert!(buckets.len() >= 1);

    let mut hasher = SipHasher::new();
    key.hash(&mut hasher);

    let mut h = hasher.finish();
    let mut b: i64 = -1;
    let mut j: i64 = 0;

    while j < (buckets.len() as i64) {
        b = j;
        h = h.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = (((b.wrapping_add(1)) as f64)
             * ((1i64 << 31) as f64)
             / (((h >> 33).wrapping_add(1)) as f64)) as i64;
    }

    buckets.iter().skip(b as usize).next().unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;
    use super::*;

    #[test]
    fn jumping() {
        let mut buckets = BTreeSet::new();
        buckets.insert("zero");
        buckets.insert("one");
        buckets.insert("two");
        buckets.insert("three");

        let before: Vec<&str> = (0..5).map(|i| select(&i, &buckets)).cloned().collect();
        assert_eq!(before, vec!["one", "three", "zero", "zero", "three"]);

        buckets.remove(&"zero");

        let after: Vec<&str> = (0..5).map(|i| select(&i, &buckets)).cloned().collect();
        assert_eq!(after, vec!["one", "three", "one", "three", "three"]);
    }
}
