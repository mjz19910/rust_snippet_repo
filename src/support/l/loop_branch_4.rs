use crate::disabled;

use super::p_dbg;
use super::ptr_math::add;
use super::{ptr_iter::PtrIter, LoopState};

use LoopState::LoopContinue;

pub fn loop_branch_4(state: &mut PtrIter, value: (*const u8, usize, u32, u32)) -> LoopState {
    disabled!(println!("{} str_ptr: {:x?}", p_dbg(state), value.0));
    add(&mut state.fns_arr, 1);
    LoopContinue
}
