pub fn elf_base<T>(origin: *const u8, _0: *const T) -> isize {
    unsafe { (_0 as *const u8).offset_from(origin) }
}
