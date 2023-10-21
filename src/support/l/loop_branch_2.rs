use crate::disabled;
use crate::support::debug_location_value;

use super::ptr_math::add;
use super::{get_type, ptr_iter::PtrIter, LoopState};
use std::os::unix::ffi::OsStrExt;
use std::{ffi::OsStr, slice::from_raw_parts};

use LoopState::LoopContinue;

pub fn loop_branch_2(state: &mut PtrIter) -> LoopState {
    let value: (*const u8, usize, u32, u32) = get_type(state.fns_arr);
    let slice = unsafe { from_raw_parts(value.0, value.1) };
    let os_str = OsStr::from_bytes(slice);
    if let Some(str_v) = os_str.to_str() {
        disabled!(debug_location_value(state, str_v, value));
    }
    add(&mut state.fns_arr, 3);
    LoopContinue
}
