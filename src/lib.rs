#![feature(alloc)]
#![feature(collections)]
#![feature(shared)]
#![no_std]


extern crate alloc;
extern crate collections;


mod gc_box;
mod gc_mark;
mod gc_state;
mod gc;


pub use gc_mark::GcMark;
pub use gc_state::GcState;
pub use gc::Gc;
