//! An implementation of the [Jump Consistent Hash](http://arxiv.org/abs/1406.2294) algorithm by
//! Lamping & Veach.

#![feature(core,hash)]

use std::hash::{Hash, Hasher, SipHasher};
use std::ops::Range;

/// Computes the bucket for the given key and given range of buckets.
///
/// # Panics
//
/// * `hash` will panic if the range of buckets is empty.
pub fn hash<T: Hash>(key: &T, buckets: Range<u32>) -> u32 {
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
    b as u32 + buckets.start
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn jumping() {
        let idx = hash(&"woo", 0..100);
        assert_eq!(idx, 79);
    }
}
