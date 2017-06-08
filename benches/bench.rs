#![feature(test)]


extern crate test;

extern crate gc;


use test::Bencher;

use gc::Gc;


#[bench]
fn bench_sgc(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..32 {
            let a = Gc::new(i);

            for _ in 0..32 {
                assert_eq!(*a, i);
            }
        }
    });
}
