use alloc::boxed::Box;

use collections::string::String;


pub trait GcMark {
    #[inline(always)]
    fn gc_unroot(&self) {}
    #[inline(always)]
    fn gc_mark(&self) {}
}


macro_rules! trait_gc_mark {
    ($($T:ty),*) => {
        $(
            impl GcMark for $T {}
        )*
    }
}

trait_gc_mark!(
    u8, u16, u32, u64, usize,
    i8, i16, i32, i64, isize,
    bool,
    char,
    String
);

impl<'a> GcMark for &'a str {}

impl<T: GcMark> GcMark for Box<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        GcMark::gc_unroot(self.as_ref())
    }
    #[inline(always)]
    fn gc_mark(&self) {
        GcMark::gc_mark(self.as_ref())
    }
}

impl<T: GcMark> GcMark for Option<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        match self {
            &Some(ref value) => GcMark::gc_unroot(value),
            &None => (),
        }
    }
    #[inline(always)]
    fn gc_mark(&self) {
        match self {
            &Some(ref value) => GcMark::gc_mark(value),
            &None => (),
        }
    }
}
