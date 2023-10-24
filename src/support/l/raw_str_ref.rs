use std::{ffi::OsStr, os::unix::prelude::OsStrExt, slice::from_raw_parts};

use super::{elf_base, get_debug_flag_state, PtrIter};

#[derive(Copy, Clone, Debug)]
pub struct RawStrRef(*const u8, usize);

impl RawStrRef {
    pub fn debug(&self, iter: &PtrIter) {
        if !get_debug_flag_state() {
            return;
        }
        println!(
            "{} RawStrRef::debug(): ({:?}@{:#x})",
            iter.p_dbg(),
            self.to_str(),
            self.elf_base_from(iter.elf_origin),
        );
    }
    pub fn as_os_str(&self) -> &OsStr {
        let slice = unsafe { from_raw_parts(self.0, self.1) };
        OsStr::from_bytes(slice)
    }
    pub fn to_str(&self) -> &str {
        self.as_os_str().to_str().unwrap()
    }
    pub fn elf_base_from(&self, origin: *const u8) -> isize {
        elf_base(origin, self.0)
    }
    pub fn before0(&self, origin: *const u8) -> bool {
        self.0 < origin
    }
    pub fn after0(&self, origin: *const u8) -> bool {
        self.0 > origin
    }
    pub fn after1(&self, origin: *const u8) -> bool {
        self.as_ptr_pair().1 > origin
    }
    pub fn str_ptr(&self) -> String {
        format!("{:x?}", self.0)
    }
    pub fn as_ptr_pair(&self) -> &(*const u8, *const u8) {
        let v = self as *const _ as *const (*const u8, *const u8);
        unsafe { &*v }
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.0
    }
}
