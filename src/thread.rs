use std::cell::RefCell;

use super::gc_state::GcState;


thread_local!(pub static GC_STATE: RefCell<GcState> = RefCell::new(GcState::new()));
