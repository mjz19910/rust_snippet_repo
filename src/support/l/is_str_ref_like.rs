use super::PtrIter;

pub fn is_str_ref_like(state: &PtrIter, value: (*const u8, usize)) -> bool {
    if value.0 < state.last_func_ptr {
        return false;
    }
    value.1 < 0x2000
}
