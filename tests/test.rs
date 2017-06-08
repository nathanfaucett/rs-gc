extern crate gc;


use gc::{GcState, Gc, GcMark, GC_STATE};


#[test]
fn test_simple() {
    let mut gc_state = GcState::new();

    let a = Gc::new_with_gc_state(&mut gc_state, 'a');

    {
        let b = Gc::new_with_gc_state(&mut gc_state, 'b');
        assert_eq!(gc_state.bytes_allocated(), 64);
        assert_eq!(*b, 'b');
    }

    gc_state.mark_and_sweep();

    assert_eq!(gc_state.bytes_allocated(), 32);
    assert_eq!(*a, 'a');
}

struct Foo<T: GcMark> {
    bar: Gc<T>,
}

impl<T: GcMark> GcMark for Foo<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        GcMark::gc_unroot(&self.bar)
    }
    #[inline(always)]
    fn gc_mark(&self) {
        GcMark::gc_mark(&self.bar)
    }
}

impl<T: GcMark> Drop for Foo<T> {
    fn drop(&mut self) {
        println!("drop");
    }
}

#[test]
fn test_comple() {
    let mut gc_state = GcState::new();

    let a = Gc::new_with_gc_state(&mut gc_state, 'a');
    let gc_a = Gc::new_with_gc_state(&mut gc_state, Foo { bar: a });

    {
        let gc_b = gc_a.clone();
        assert_eq!(gc_state.bytes_allocated(), 72);
        assert_eq!(*gc_b.bar, 'a');
    }

    gc_state.mark_and_sweep();

    assert_eq!(gc_state.bytes_allocated(), 72);
    assert_eq!(*gc_a.bar, 'a');
}

#[test]
fn test_thrashing() {
    let mut gc_state = GcState::new();
    let mut count = 0;

    for i in 0..32 {
        let bar = Gc::new_with_gc_state(&mut gc_state, i);

        for _ in 0..32 {
            let foo = Gc::new_with_gc_state(&mut gc_state, Foo {
                bar: bar.clone(),
            });

            for _ in 0..32 {
                count += *foo.bar;
            }
        }
    }

    assert_eq!(gc_state.bytes_allocated(), 112);
    gc_state.mark_and_sweep();
    assert_eq!(gc_state.bytes_allocated(), 0);

    assert_eq!(count, 507904);
}

#[test]
fn test_threading() {
    use std::thread;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let count = Arc::new(AtomicUsize::new(0));

    for i in 0..32 {
        let bar = Gc::new(i);
        let c = count.clone();

        let handle = thread::spawn(move || {
            for _ in 0..32 {
                let foo = Gc::new(Foo {
                    bar: bar.clone(),
                });

                for _ in 0..32 {
                    c.fetch_add(*foo.bar, Ordering::Relaxed);
                }
            }
        });

        let _ = handle.join();
    }

    GC_STATE.with(|state| {
        let mut state = state.borrow_mut();

        assert_eq!(state.bytes_allocated(), 160);
        state.mark_and_sweep();
        assert_eq!(state.bytes_allocated(), 0);
    });

    assert_eq!(count.load(Ordering::Relaxed), 507904);
}
