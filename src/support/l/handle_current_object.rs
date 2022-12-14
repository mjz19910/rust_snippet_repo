use crate::support::get_type::get_type;
use crate::support::print_debug_state::print_debug_state;
use crate::{disabled, support::metadata::XVTable};

use super::{loop_state::LoopState, metadata::GetX, ptr_iter::PtrIter};

pub fn handle_current_object<const N: usize>(
    state: &mut PtrIter,
    get_x: &mut Option<Box<dyn GetX>>,
) -> LoopState {
    let value: XVTable<(), N> = get_type(state.fns_arr);
    get_x.replace(Box::new(value));
    disabled!(print_debug_state(state, N, value, "A"));
    use crate::support::ptr_math::add;
    add(&mut state.fns_arr, 3 + N);
    LoopState::LoopContinue
}
