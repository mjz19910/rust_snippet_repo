use std::{ffi::OsStr, os::unix::prelude::OsStrExt, slice::from_raw_parts};

use super::elf_base;

#[derive(Copy, Clone, Debug)]
pub struct RawStrRef(*const u8, usize);

impl RawStrRef {
    pub fn as_os_str(&self) -> &OsStr {
        let slice = unsafe { from_raw_parts(self.0, self.1) };
        OsStr::from_bytes(slice)
    }
    pub fn to_str(&self) -> Option<&str> {
        self.as_os_str().to_str()
    }
    pub fn elf_base_from(&self, ptr: *const u8) -> isize {
        elf_base(ptr, self.0)
    }
    pub fn before0(&self, ptr: *const u8) -> bool {
        self.0 < ptr
    }
    pub fn after0(&self, ptr: *const u8) -> bool {
        self.0 > ptr
    }
    pub fn after1(&self, ptr: usize) -> bool {
        self.1 > ptr
    }
    pub fn str_ptr(&self) -> String {
        format!("{:x?}", self.0)
    }
}
