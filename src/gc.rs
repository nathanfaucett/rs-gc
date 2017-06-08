use core::ops::Deref;
use core::fmt;
use core::ptr::Shared;

use super::gc_box::GcBox;
use super::gc_mark::GcMark;
use super::gc_state::GcState;


pub struct Gc<T: GcMark + ?Sized> {
    gc_box: Shared<GcBox<T>>,
}

impl<T: GcMark + 'static> Gc<T> {

    #[inline]
    pub fn new_with_gc_state(gc_state: &mut GcState, value: T) -> Self {
        let gc_box = gc_state.new_gc_box(value);

        unsafe {
            let gc_box_mut = &mut (*gc_box.as_ptr());
            gc_box_mut.as_ref().gc_unroot();
        }

        Self::from_gc_box(gc_box)
    }
}

impl<T: GcMark + ?Sized> Gc<T> {

    #[inline(always)]
    pub(crate) fn from_gc_box(gc_box: Shared<GcBox<T>>) -> Self {
        Gc {
            gc_box: gc_box,
        }
    }

    #[inline(always)]
    fn gc_box_as_ref(&self) -> &GcBox<T> {
        unsafe {
            &(*self.gc_box.as_ptr())
        }
    }

    #[inline(always)]
    pub fn as_ref(&self) -> &T {
        self.gc_box_as_ref().as_ref()
    }
}

impl<T: GcMark + ?Sized> Clone for Gc<T> {
    #[inline]
    fn clone(&self) -> Self {
        let gc = Self::from_gc_box(self.gc_box);
        gc.gc_box_as_ref().inc_roots();
        gc
    }
}

impl<T: GcMark + ?Sized> Drop for Gc<T> {
    #[inline(always)]
    fn drop(&mut self) {
        self.gc_box_as_ref().dec_roots();
    }
}

impl<T: GcMark + ?Sized> GcMark for Gc<T> {
    #[inline(always)]
    fn gc_unroot(&self) {
        unsafe {
            &(*self.gc_box.as_ptr()).gc_unroot();
        }
    }
    #[inline(always)]
    fn gc_mark(&self) {
        unsafe {
            &(*self.gc_box.as_ptr()).gc_mark();
        }
    }
}

impl<T: GcMark + ?Sized> Deref for Gc<T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.as_ref()
    }
}

impl<T: fmt::Debug + GcMark + ?Sized> fmt::Debug for Gc<T> {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_ref())
    }
}

impl<T: fmt::Display + GcMark + ?Sized> fmt::Display for Gc<T> {

    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_ref())
    }
}
