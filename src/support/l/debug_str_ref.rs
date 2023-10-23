use super::{elf_base, p_dbg, PtrIter};

pub fn debug_str_ref(state: &PtrIter, str_v: &str, value: (*const u8, usize)) {
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v
    );
}
