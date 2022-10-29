use crate::support::{self, LAZY_OFFSETS_SET};

use super::ptr_iter::PtrIter;

pub fn mark_offset_hit(state: &PtrIter, opt: bool) {
    use support::constants::SKIP_CODE_GEN;
    let shl = |v: i32, o: i32| v << o;
    let cfg_hit_code_gen = |ex: &str| {
        let set = unsafe { &LAZY_OFFSETS_SET };
        let cur_len = set.0.borrow().len();
        if cur_len < 0x80 {
            println!(
                "        {:#x} => true, //d:{}:{:02x}{ex};",
                state.cur_offset,
                shl(cfg!(feature = "code_gen").into(), 0)
                    + shl(cfg!(feature = "debug").into(), 1)
                    + shl((state.is_debug_build == 0).into(), 2),
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
        let do_code_gen_config = cfg!(feature = "code_gen") || state.runtime_code_gen_flag;
        if do_code_gen_config && unsafe { !SKIP_CODE_GEN } {
            cfg_hit_code_gen(if opt { " " } else { "" });
        }
    }
}
