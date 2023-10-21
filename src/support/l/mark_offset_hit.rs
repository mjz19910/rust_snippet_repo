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

pub fn mark_offset_hit(state: &PtrIter, opt: bool) {
    let cfg_hit_code_gen = |ex: &str| {
        let set = unsafe { &LAZY_OFFSETS_SET };
        let cur_len = set.0.borrow().len();
        if cur_len < 0x80 {
            println!(
                "        {:#x} => true, //d:{}:{:02x}{ex};",
                state.cur_offset,
                Into::<i32>::into(state.is_debug_build == 0),
                cur_len
            );
        }
    };
    let has_offset = {
        let set = unsafe { &LAZY_OFFSETS_SET };
        set.0.borrow().contains(&state.cur_offset)
    };
    if !has_offset {
        {
            let set = unsafe { &LAZY_OFFSETS_SET };
            set.0.borrow_mut().insert(state.cur_offset);
        }
        let do_code_gen_config = state.runtime_code_gen_flag;
        if do_code_gen_config {
            cfg_hit_code_gen(if opt { " " } else { "" });
        }
    }
}
