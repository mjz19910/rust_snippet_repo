use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::slice;

mod l;
pub use l::*;
mod s;
pub use s::*;

use LoopState::LoopContinue;

use crate::disabled;

use self::ptr_iter::PtrIter;

#[derive(Clone, Copy, Debug)]
pub(crate) struct RawLocation(*const u8, usize, u32, u32);

pub(crate) fn get_location(fns_arr: *const *const ()) -> RawLocation {
    get_type(fns_arr)
}

pub(crate) fn is_location_str(state: &PtrIter, value: RawLocation) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    value.3 != 0 && value.3 < 0x1000
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct RawStrRef(*const u8, usize);

pub(crate) fn is_str_ref_like(state: &PtrIter, value: RawStrRef) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    value.1 < 0x2000
}

pub(crate) fn get_str_ref(fns_arr: *const *const ()) -> RawStrRef {
    get_type(fns_arr)
}

fn check_vtable_size_of(state: &PtrIter, size_of: usize) -> bool {
    match size_of {
        0x8 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0x40 | 0x48 | 0x50 => return true,
        _ => (),
    }
    if size_of < 0x1000 {
        println!(
            "{} find_next_object(new_size): {:#x}",
            p_dbg(state),
            size_of
        );
        return true;
    }
    false
}

fn is_cached_offset(state: &PtrIter) -> bool {
    matches!(state.cur_offset, 0)
}

fn loop_branch_4(state: &mut PtrIter, value: (*const u8, usize, u32, u32)) -> LoopState {
    disabled!(println!("{} str_ptr: {:x?}", p_dbg(state), value.0));
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 1);
    LoopContinue
}

fn debug_location_value(state: &PtrIter, str_v: &str, value: (*const u8, usize, u32, u32)) {
    println!(
        "{} debug_location_value: ({:#x}, {:?}, {:#05x}, {:#04x})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v,
        value.2,
        value.3,
    );
}

fn loop_branch_2(state: &mut PtrIter) -> LoopState {
    let value: (*const u8, usize, u32, u32) = get_type(state.fns_arr);
    let slice = unsafe { slice::from_raw_parts(value.0, value.1) };
    let os_str = OsStr::from_bytes(slice);
    if let Some(str_v) = os_str.to_str() {
        disabled!(debug_location_value(state, str_v, value));
    }
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 3);
    LoopContinue
}

fn debug_str_ref(state: &PtrIter, str_v: &str, value: RawStrRef) {
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v
    );
}

fn loop_branch_1(state: &mut PtrIter) -> LoopState {
    let value: RawStrRef = get_type(state.fns_arr);
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 2);
    let res = unsafe { slice::from_raw_parts(value.0, value.1) };
    let os_str = OsStr::from_bytes(res);
    if let Some(str_v) = os_str.to_str() {
        disabled!(debug_str_ref(state, str_v, value));
    }
    LoopContinue
}
