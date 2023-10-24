use super::PtrIter;

pub fn check_vtable_size_of(state: &PtrIter, size_of: usize) -> bool {
    match size_of {
        0x8 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 | 0x40 | 0x48 | 0x50 => return true,
        _ => (),
    }
    if size_of < 0x1000 {
        println!(
            "{} find_next_object(new_size): {:#x}",
            state.p_dbg(),
            size_of
        );
        return true;
    }
    false
}
