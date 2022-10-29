fn elf_base_private<T>(start: *const u8, end: *const T) -> isize {
    unsafe { (end as *const u8).offset_from(start) }
}

pub fn elf_base<T>(start: *const u8, end: *const T) -> isize {
    elf_base_private(start, end)
}
