use super::{PtrIter, RawLocation};

pub(crate) fn is_location_str(state: &PtrIter, value: RawLocation) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    value.3 != 0 && value.3 < 0x1000
}
