use super::{PtrIter, RawStrRef};

pub(crate) fn is_str_ref_like(state: &PtrIter, value: RawStrRef) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    value.1 < 0x2000
}
