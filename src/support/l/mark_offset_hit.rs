use std::{
    cell::{LazyCell, RefCell},
    collections::HashSet,
};

use super::ptr_iter::PtrIter;

pub struct OffSTy(LazyCell<RefCell<HashSet<isize>>>);

impl OffSTy {
    const fn new() -> Self {
        let init = || RefCell::new(HashSet::new());
        Self(LazyCell::new(init))
    }
}

pub static mut LAZY_OFFSETS_SET: OffSTy = OffSTy::new();

pub fn mark_offset_hit(state: &PtrIter) {
    let has_offset = {
        let set = unsafe { &LAZY_OFFSETS_SET };
        set.0.borrow().contains(&state.cur_offset)
    };
    if !has_offset {
        {
            let set = unsafe { &LAZY_OFFSETS_SET };
            set.0.borrow_mut().insert(state.cur_offset);
        }
    }
}
