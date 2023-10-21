use crate::{
    disabled,
    support::{
        elf_base, is_cached_offset, iter_find_next_object, mark_offset_hit,
        metadata::{GetX, XVTable},
    },
};

use super::{
    p_dbg,
    ptr_iter::PtrIter,
    LoopState::{self, LoopBreak, LoopContinue},
};

pub fn loop_inner_3(state: &mut PtrIter) -> LoopState {
    let opt = is_cached_offset(state);
    mark_offset_hit(state, opt);
    const N: usize = 3;
    let value: XVTable<(), N> = crate::support::get_type(state.fns_arr);
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
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 6);
    LoopBreak
}
