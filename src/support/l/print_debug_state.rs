use super::ptr_iter::PtrIter;
use std::fmt::Debug;

pub fn print_debug_state<T>(state: &PtrIter, num: usize, value: T, name: &str)
where
    T: Debug,
{
    println!(
        "{} p_dbg_ptr: {}({}, {:#x})",
        state.p_dbg(),
        name,
        num,
        state.cur_offset,
    );
    println!("{} p_dbg_vtb: {:x?}", state.p_dbg(), value);
}
