use super::{
    get_type,
    metadata::{GetX, XVTable},
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
    value.debug(state, "A");
    add(&mut state.fns_arr, 3 + N);
    LoopContinue
}
