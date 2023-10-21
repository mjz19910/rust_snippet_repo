use super::{elf_base, p_dbg, PtrIter, RawStrRef};

pub(crate) fn debug_str_ref(state: &PtrIter, str_v: &str, value: RawStrRef) {
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v
    );
}
