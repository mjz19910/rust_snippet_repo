use crate::support::p_dbg::p_dbg;
use crate::{
    disabled,
    support::{
        get_type::get_type, loop_branch_1, loop_branch_2, loop_branch_4, loop_state::LoopState,
    },
};

use super::ptr_iter::PtrIter;

pub fn loop_inner_1(state: &mut PtrIter) -> LoopState {
    use LoopState::LoopBreak;
    let value: (*const u8, usize, u32, u32) = get_type(state.fns_arr);
    use crate::support::elf_base::elf_base;
    state.ptr_base = elf_base(state.elf_base_ptr, value.0);
    state.cur_offset = state.ptr_base - state.start_count[0];
    if (value.0 as usize) < (state.elf_base_ptr as usize) {
        return LoopBreak;
    }
    disabled!(println!("{} loop_iter: {:x?}", p_dbg(state), value));
    if value.0 > state.elf_base_ptr && value.1 > (state.elf_base_ptr as usize) {
        return loop_branch_4(state, value);
    }
    if value.0 < state.last_func_ptr {
        use crate::support::loop_inner_3::loop_inner_3;
        return loop_inner_3(state);
    }
    if value.3 < 0x1000 {
        return loop_branch_2(state);
    }
    if state.cur_offset > 0x1000 {
        return loop_branch_1(state);
    }
    println!("loop_inner_1(break): {} {:x?}", p_dbg(state), value);
    LoopBreak
}
