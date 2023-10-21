use super::{
    find_next_object, handle_current_object, loop_state::LoopState, metadata::GetX,
    ptr_iter::PtrIter,
};

pub fn iter_find_next_object(state: &mut PtrIter, get_x: &mut Option<Box<dyn GetX>>) -> LoopState {
    {
        const N: usize = 1;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 2;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 3;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 4;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 5;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 6;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    {
        const N: usize = 7;
        if find_next_object::<N>(state) {
            return handle_current_object::<N>(state, get_x);
        }
    }
    LoopState::LoopBreak
}
