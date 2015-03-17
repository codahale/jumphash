#![feature(test)]

extern crate jumphash;
extern crate test;

use jumphash::hash;
use test::Bencher;

#[bench]
fn jumphash_hash(b: &mut Bencher) {
    let key = &"woo hoo";

    b.iter(|| {
        hash(key, 100)
    })
}
