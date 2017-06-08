use core::fmt;
use core::ptr::Shared;

use super::gc_mark::GcMark;


pub struct GcBox<T: GcMark + ?Sized> {
    roots: usize,
    marked: bool,
    next: Option<Shared<GcBox<GcMark>>>,
    data: T
}

impl<T: GcMark> GcBox<T> {

    #[inline(always)]
    pub fn new(value: T, next: Option<Shared<GcBox<GcMark>>>) -> Self {
        GcBox {
            roots: 1usize,
            marked: false,
            next: next,
            data: value
        }
    }
}

impl<T: GcMark + ?Sized> GcBox<T> {

    #[inline(always)]
    pub fn next(&self) -> Option<Shared<GcBox<GcMark>>> {
        self.next
    }
    #[inline(always)]
    pub fn set_next(&mut self, next: Option<Shared<GcBox<GcMark>>>) {
        self.next = next;
    }

    #[inline(always)]
    pub fn roots(&self) -> usize {
        self.roots
    }
    #[inline(always)]
    pub fn inc_roots(&mut self) {
        self.roots = self.roots.checked_add(1usize).expect("roots overflow");
    }
    #[inline(always)]
    pub fn dec_roots(&mut self) {
        self.roots = self.roots.checked_sub(1usize).expect("roots overflow");
    }

    #[inline(always)]
    fn unroot(&mut self) {
        self.as_ref().gc_unroot();
        self.dec_roots();
    }

    #[inline(always)]
    fn mark(&mut self) {
        if !self.marked {
            self.marked = true;
            self.data.gc_mark();
        }
    }
    #[inline(always)]
    pub fn unmark(&mut self) {
        self.marked = false;
    }
    #[inline(always)]
    pub fn is_marked(&self) -> bool {
        self.marked
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: GcMark + ?Sized> GcMark for GcBox<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };
        self_mut.unroot();
    }
    #[inline(always)]
    fn gc_mark(&self) {
        let self_mut = unsafe {
            &mut *(self as *const Self as *mut Self)
        };
        self_mut.mark();
    }
}

impl<T: fmt::Display + GcMark + ?Sized> fmt::Display for GcBox<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.data)
    }
}

impl<T: fmt::Debug + GcMark + ?Sized> fmt::Debug for GcBox<T> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.data)
    }
}
