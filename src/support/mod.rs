pub mod l;
pub mod s;
use std::cell::{LazyCell, RefCell};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::slice;

use l::loop_state::LoopState;
pub use l::*;
pub use s::*;

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
        use crate::support::p_dbg::p_dbg;
        println!(
            "{} find_next_object(new_size): {:#x}",
            p_dbg(state),
            size_of
        );
        return true;
    }
    false
}

pub struct OffSTy(LazyCell<RefCell<HashSet<isize>>>);

impl OffSTy {
    const fn new() -> Self {
        let init = || RefCell::new(HashSet::new());
        Self(LazyCell::new(init))
    }
}

pub static mut LAZY_OFFSETS_SET: OffSTy = OffSTy::new();

fn is_cached_offset(state: &PtrIter) -> bool {
    matches!(state.cur_offset, 0)
}

fn loop_branch_4(state: &mut PtrIter, value: (*const u8, usize, u32, u32)) -> LoopState {
    use crate::support::p_dbg::p_dbg;
    use LoopState::LoopContinue;
    disabled!(println!("{} str_ptr: {:x?}", p_dbg(state), value.0));
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 1);
    LoopContinue
}

fn debug_location_value(state: &PtrIter, str_v: &str, value: (*const u8, usize, u32, u32)) {
    use crate::support::elf_base::elf_base;
    use crate::support::p_dbg::p_dbg;
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
    use LoopState::LoopContinue;
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
    use crate::support::elf_base::elf_base;
    use crate::support::p_dbg::p_dbg;
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v
    );
}

fn loop_branch_1(state: &mut PtrIter) -> LoopState {
    use LoopState::LoopContinue;
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

use crate::disabled;
use crate::support::get_type::get_type;

use self::ptr_iter::PtrIter;
