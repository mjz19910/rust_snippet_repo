use crate::{
    disabled,
    support::{metadata::XVTable, print_debug_state},
};

use super::{loop_state::LoopState, metadata::GetX, ptr_iter::PtrIter};

pub fn handle_current_object<const N: usize>(
    state: &mut PtrIter,
    get_x: &mut Option<Box<dyn GetX>>,
) -> LoopState {
    let value: XVTable<(), N> = crate::support::get_type(state.fns_arr);
    get_x.replace(Box::new(value));
    disabled!(print_debug_state(state, N, value, "A"));
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 3 + N);
    LoopState::LoopContinue
}
