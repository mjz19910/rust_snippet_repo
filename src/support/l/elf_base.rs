#[allow(clippy::missing_safety_doc)]
pub unsafe fn elf_base<T>(elf_base: *const u8, elf_offset_ptr: *const T) -> isize {
    unsafe { (elf_offset_ptr as *const u8).offset_from(elf_base) }
}
