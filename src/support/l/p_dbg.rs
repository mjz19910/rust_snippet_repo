use super::ptr_iter::PtrIter;

pub fn p_dbg(v: &PtrIter) -> &'static str {
    if v.is_debug_build == 1 {
        "D"
    } else {
        "R"
    }
}
