use crate::disabled;

use super::{
    get_type,
    metadata::{GetX, XVTable},
    print_debug_state,
    ptr_iter::PtrIter,
    ptr_math::add,
    LoopState::{self, LoopContinue},
};

pub fn handle_current_object<const N: usize>(
    state: &mut PtrIter,
    get_x: &mut Option<Box<dyn GetX>>,
) -> LoopState {
    let value: XVTable<(), N> = get_type(state.fns_arr);
    get_x.replace(Box::new(value));
    disabled!(print_debug_state(state, N, value, "A"));
    add(&mut state.fns_arr, 3 + N);
    LoopContinue
}
