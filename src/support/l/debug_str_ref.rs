use super::{PtrIter, RawStrRef};

pub fn debug_str_ref(state: &PtrIter, str_v: &str, value: RawStrRef) {
    println!(
        "{} debug_str_ref: ({:#x}, {:?})",
        state.p_dbg(),
        value.elf_base_from(state.elf_origin),
        str_v
    );
}
