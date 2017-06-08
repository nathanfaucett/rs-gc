#![feature(test)]


extern crate test;

extern crate gc;


use test::Bencher;

use gc::{Gc, GcState};


#[bench]
fn bench_gc(b: &mut Bencher) {
    let mut gc_state = GcState::new();

    b.iter(|| {
        for _ in 0..32 {
            for i in 0..32 {
                let _ = Gc::new_with_gc_state(&mut gc_state, i);
            }
        }
    });
}

#[bench]
fn bench_local_thread_gc(b: &mut Bencher) {
    b.iter(|| {
        for _ in 0..32 {
            for i in 0..32 {
                let _ = Gc::new(i);
            }
        }
    });
}
