use crate::{
    disabled,
    support::{
        check_vtable_size_of, get_location, get_str_ref, get_type, is_location_str,
        is_str_ref_like, metadata::XVTable, p_dbg::p_dbg,
    },
};

use super::ptr_iter::PtrIter;

pub fn find_next_object<const N: usize>(state: &mut PtrIter) -> bool
where
    [(); N + 3]:,
{
    let value: XVTable<(), { N + 3 }> = get_type::get_type(state.fns_arr);
    let mut fns_arr_cur = state.fns_arr;
    use crate::support::ptr_math::add;
    add(&mut fns_arr_cur, N + 3);
    let next_is_location = is_location_str(state, get_type::get_type(fns_arr_cur));
    if next_is_location {
        disabled!(println!(
            "{} find_next_object: {:x?}",
            p_dbg(state),
            get_location(fns_arr_cur)
        ));
        return true;
    }
    let next_is_str_desc = is_str_ref_like(state, get_type::get_type(fns_arr_cur));
    let val = &value.vtable_fns.map(|x| x as usize)[N..];
    if next_is_str_desc {
        disabled!(println!(
            "{} find_next_object(str_ref): {:x?}",
            p_dbg(state),
            get_str_ref(fns_arr_cur)
        ));
        return true;
    }
    let next_is_vtable = match val[1..] {
        [0x1, 0x1] | [0x0, 0x1] => true,
        [size_of, 0x8] => check_vtable_size_of(state, size_of),
        _ => false,
    };
    if next_is_vtable {
        disabled!(println!(
            "{} find_next_object: IsTable(value: {:x?})",
            p_dbg(state),
            val
        ));
        return true;
    }
    false
}
