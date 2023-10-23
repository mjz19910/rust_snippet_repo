use super::{p_dbg, PtrIter, RawStrRef};

pub fn debug_str_ref(state: &PtrIter, str_v: &str, value: RawStrRef) {
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        p_dbg(state),
        value.elf_base_from(state.elf_base_ptr),
        str_v
    );
}
