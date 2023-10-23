fn elf_base_private<T>(ptr: *const u8, end: *const T) -> isize {
    unsafe { (end as *const u8).offset_from(ptr) }
}

pub fn elf_base<T>(ptr: *const u8, end: *const T) -> isize {
    elf_base_private(ptr, end)
}
