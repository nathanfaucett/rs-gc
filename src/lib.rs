#![feature(heap_api)]
#![feature(alloc)]
#![feature(collections)]
#![feature(shared)]
#![no_std]


#[cfg(not(feature = "no_std"))]
#[macro_use]
extern crate std;


extern crate alloc;
extern crate collections;


mod gc_box;
mod gc_mark;
mod gc_state;
mod gc;
#[cfg(not(feature = "no_std"))]
mod thread;


pub use gc_mark::GcMark;
pub use gc_state::GcState;
pub use gc::Gc;
#[cfg(not(feature = "no_std"))]
pub use thread::GC_STATE;
