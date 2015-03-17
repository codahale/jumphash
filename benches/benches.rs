#![feature(test)]

extern crate jumphash;
extern crate test;

use std::collections::BTreeSet;
use jumphash::select;
use test::Bencher;

#[bench]
fn jumphash_select(b: &mut Bencher) {
    let key = "woo hoo";

    let mut buckets = BTreeSet::new();
    buckets.insert("zero");
    buckets.insert("one");
    buckets.insert("two");
    buckets.insert("three");

    b.iter(|| {
        select(&key, &buckets)
    })
}
