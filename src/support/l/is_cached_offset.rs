use super::PtrIter;

pub fn is_cached_offset(state: &PtrIter) -> bool {
	matches!(state.cur_offset, 0)
}
