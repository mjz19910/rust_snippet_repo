use std::{ffi::OsStr, os::unix::prelude::OsStrExt, slice::from_raw_parts};

use super::{elf_base, p_dbg, PtrIter};

#[derive(Clone, Copy, Debug)]
pub struct RawLocation(pub *const u8, pub usize, u32, u32);
impl RawLocation {
    pub fn as_os_str(&self) -> &OsStr {
        let slice = unsafe { from_raw_parts(self.0, self.1) };
        OsStr::from_bytes(slice)
    }
    pub fn to_str(&self) -> Option<&str> {
        self.as_os_str().to_str()
    }
    pub fn elf_base_from(&self, elf_base_ptr: *const u8) -> isize {
        elf_base(elf_base_ptr, self.0)
    }
    pub fn debug(&self, state: &PtrIter, str_v: &str) {
        println!(
            "{} debug_location_value: ({:#x}, {:?}, {:#05x}, {:#04x})",
            p_dbg(state),
            self.elf_base_from(state.elf_base_ptr),
            str_v,
            self.2,
            self.3,
        );
    }
    pub fn is_small(&self) -> bool {
        self.3 < 0x1000
    }
    pub fn is_empty(&self) -> bool {
        self.3 == 0
    }
}
