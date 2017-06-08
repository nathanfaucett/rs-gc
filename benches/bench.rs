#![feature(test)]


extern crate test;

extern crate gc;


use test::Bencher;

use gc::{Gc, GcMark, GcState};


#[bench]
fn bench_sgc(b: &mut Bencher) {

    struct Foo<T: GcMark> {
        bar: Gc<T>,
    }

    impl<T: GcMark> GcMark for Foo<T> {
        #[inline(always)]
        fn gc_root(&self) {
            GcMark::gc_root(&self.bar)
        }
        #[inline(always)]
        fn gc_unroot(&self) {
            GcMark::gc_unroot(&self.bar)
        }
        #[inline(always)]
        fn gc_mark(&self) {
            GcMark::gc_mark(&self.bar)
        }
    }

    let mut gc_state = GcState::new();

    b.iter(|| {
        let a = Gc::new_with_gc_state(&mut gc_state, 0);
        let gc_a = Gc::new_with_gc_state(&mut gc_state, Foo { bar: a });

        {
            let _ = gc_a.clone();
        }
    });
}
