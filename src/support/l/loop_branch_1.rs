use crate::disabled;
use crate::support::{debug_str_ref, RawStrRef};

use std::os::unix::ffi::OsStrExt;
use std::{ffi::OsStr, slice::from_raw_parts};

use super::{
    get_type,
    ptr_iter::PtrIter,
    ptr_math::add,
    LoopState::{self, LoopContinue},
};

pub fn loop_branch_1(state: &mut PtrIter) -> LoopState {
    let value: RawStrRef = get_type(state.fns_arr);
    add(&mut state.fns_arr, 2);
    let res = unsafe { from_raw_parts(value.0, value.1) };
    let os_str = OsStr::from_bytes(res);
    if let Some(str_v) = os_str.to_str() {
        disabled!(debug_str_ref(state, str_v, value));
    }
    LoopContinue
}
