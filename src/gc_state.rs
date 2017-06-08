use alloc::heap;
use alloc::boxed::Box;

use core::mem;
use core::ptr::Shared;

use super::gc_box::GcBox;
use super::gc_mark::GcMark;


const INITIAL_THRESHOLD: usize = 128_usize;
const USED_SPACE_RATIO: f64 = 0.7_f64;


pub struct GcState {
    gc_box_root: Option<Shared<GcBox<GcMark>>>,
    bytes_allocated: usize,
    threshold: usize,
}

impl GcState {

    #[inline(always)]
    pub fn new() -> Self {
        GcState {
            gc_box_root: None,
            bytes_allocated: 0usize,
            threshold: INITIAL_THRESHOLD,
        }
    }

    #[inline]
    pub(crate) fn new_gc_box<T: GcMark + 'static>(&mut self, value: T) -> Shared<GcBox<T>> {

        if self.bytes_allocated > self.threshold {
            self.mark_and_sweep();

            if self.bytes_allocated as f64 > self.threshold as f64 * USED_SPACE_RATIO {
                self.threshold = (self.bytes_allocated as f64 / USED_SPACE_RATIO) as usize;
            }
        }

        let gc_box = Box::into_raw(Box::new(
            GcBox::new(value, self.gc_box_root)
        ));
        let gc_box_shared = unsafe {
            Shared::new(gc_box)
        };

        self.gc_box_root = Some(gc_box_shared);
        self.bytes_allocated += mem::size_of::<GcBox<T>>();

        gc_box_shared
    }

    #[inline(always)]
    pub fn bytes_allocated(&self) -> usize {
        self.bytes_allocated
    }

    #[inline]
    fn mark(&mut self) {
        let mut node_option = self.gc_box_root;

        while let Some(node) = node_option {
            let gc_box = unsafe { &*node.as_ptr() };

            if gc_box.roots() > 0 {
                gc_box.gc_mark();
            }

            node_option = gc_box.next();
        }
    }

    #[inline]
    fn sweep(&mut self) {
        let mut node_option = self.gc_box_root;
        let mut prev_node_option: Option<Shared<GcBox<GcMark>>> = None;
        let mut new_gc_box_root = node_option;

        while let Some(node) = node_option {
            let gc_box = unsafe { &mut *node.as_ptr() };

            if gc_box.is_marked() {
                gc_box.unmark();
                prev_node_option = node_option;
                node_option = gc_box.next();
            } else {
                let next_option = gc_box.next();

                if let Some(ref mut prev_node) = prev_node_option {
                    unsafe {
                        prev_node.as_mut().set_next(next_option)
                    }
                } else {
                    new_gc_box_root = next_option;
                }

                let size = mem::size_of_val::<GcBox<_>>(gc_box);
                let align = mem::align_of_val::<GcBox<_>>(gc_box);

                self.bytes_allocated -= size;
                unsafe {
                    heap::deallocate(node.as_ptr() as *mut u8, size, align);
                }

                node_option = next_option;
            }
        }

        self.gc_box_root = new_gc_box_root;
    }

    #[inline(always)]
    pub fn mark_and_sweep(&mut self) {
        self.mark();
        self.sweep();
    }
}
