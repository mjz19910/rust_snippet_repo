use super::{PtrIter, RawLocation};

pub fn is_location_str(state: &PtrIter, value: RawLocation) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    (!value.is_empty()) && value.is_small()
}
