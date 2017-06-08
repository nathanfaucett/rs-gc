use core::fmt;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::ptr::Shared;

use super::gc_mark::GcMark;


pub struct GcBox<T: GcMark + ?Sized> {
    next: Option<Shared<GcBox<GcMark>>>,
    roots: AtomicUsize,
    marked: AtomicBool,
    data: T
}

unsafe impl<T: GcMark + ?Sized> Send for GcBox<T> {}
unsafe impl<T: GcMark + ?Sized> Sync for GcBox<T> {}

impl<T: GcMark> GcBox<T> {

    #[inline(always)]
    pub fn new(value: T, next: Option<Shared<GcBox<GcMark>>>) -> Self {
        GcBox {
            next: next,
            roots: AtomicUsize::new(1usize),
            marked: AtomicBool::new(false),
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
        self.roots.load(Ordering::SeqCst)
    }
    #[inline(always)]
    pub fn inc_roots(&self) {
        self.roots.fetch_add(1usize, Ordering::SeqCst);
    }
    #[inline(always)]
    pub fn dec_roots(&self) {
        self.roots.fetch_sub(1usize, Ordering::SeqCst);
    }

    #[inline(always)]
    fn unroot(&self) {
        self.as_ref().gc_unroot();
        self.dec_roots();
    }

    #[inline(always)]
    fn mark(&self) {
        if !self.marked.swap(true, Ordering::SeqCst) {
            self.data.gc_mark();
        }
    }
    #[inline(always)]
    pub fn unmark(&self) {
        self.marked.store(false, Ordering::SeqCst);
    }
    #[inline(always)]
    pub fn is_marked(&self) -> bool {
        self.marked.load(Ordering::SeqCst)
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: GcMark + ?Sized> GcMark for GcBox<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        self.unroot();
    }
    #[inline(always)]
    fn gc_mark(&self) {
        self.mark();
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
