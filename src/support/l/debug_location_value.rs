use super::{elf_base, p_dbg, PtrIter};

pub(crate) fn debug_location_value(
    state: &PtrIter,
    str_v: &str,
    value: (*const u8, usize, u32, u32),
) {
    println!(
        "{} debug_location_value: ({:#x}, {:?}, {:#05x}, {:#04x})",
        p_dbg(state),
        elf_base(state.elf_base_ptr, value.0),
        str_v,
        value.2,
        value.3,
    );
}
