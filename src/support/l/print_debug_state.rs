use super::{p_dbg, ptr_iter::PtrIter};
use std::fmt::Debug;

pub fn print_debug_state<T>(state: &PtrIter, num: usize, value: T, name: &str)
where
    T: Debug,
{
    println!(
        "{} p_dbg_ptr: {}({}, {:#x})",
        p_dbg(state),
        name,
        num,
        state.cur_offset,
    );
    println!("{} p_dbg_vtb: {:x?}", p_dbg(state), value);
}
