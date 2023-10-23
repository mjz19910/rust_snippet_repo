use crate::{disabled, support::is_cached_offset};

use super::{
    elf_base, get_type, iter_find_next_object, mark_offset_hit,
    metadata::{GetX, XVTable},
    p_dbg,
    ptr_iter::PtrIter,
    ptr_math::add,
    LoopState::{self, LoopBreak, LoopContinue},
};

pub fn loop_branch_3(state: &mut PtrIter) -> LoopState {
    let opt = is_cached_offset(state);
    mark_offset_hit(state, opt);
    const N: usize = 3;
    let value: XVTable<(), N> = get_type(state.fns_arr);
    let vtable_rva: [isize; N] = value.vtable_fns.map(|x| elf_base(state.elf_base_ptr, x));
    let _vtable_num: [usize; N] = value.vtable_fns.map(|x| x as usize);
    let mut get_x: Option<Box<dyn GetX>> = Some(Box::new(value));
    let result = iter_find_next_object(state, &mut get_x);
    disabled!(println!(
        "{} print_get_x_box: {}",
        p_dbg(state),
        get_x.expect("get_x is some").x_value()
    ));
    if let LoopContinue = result {
        return result;
    }
    print!("state_check_3: {} {:#x}: ", p_dbg(state), state.cur_offset);
    print!("({:x?}) ", vtable_rva);
    print!("@!(3) ");
    print!("{:x?}", value);
    println!();
    add(&mut state.fns_arr, 6);
    LoopBreak
}
