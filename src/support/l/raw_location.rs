use super::{get_debug_flag_state, PtrIter, RawStrRef};

#[derive(Clone, Copy, Debug)]
pub struct RawLocation(RawStrRef, u32, u32);
impl RawLocation {
    pub fn debug(&self, iter: &PtrIter) {
        if !get_debug_flag_state() {
            return;
        }
        let str_v = self.to_str();
        println!(
            "{} RawLocation::debug(): ({:?}@{:#x}, {:#05x}, {:#04x})",
            iter.p_dbg(),
            str_v,
            self.elf_base_from(iter.elf_origin),
            self.1,
            self.2,
        );
    }
    pub fn is_small(&self) -> bool {
        self.2 < 0x1000
    }
    pub fn is_empty(&self) -> bool {
        self.2 == 0
    }
    pub fn before0(&self, origin: *const u8) -> bool {
        self.0.before0(origin)
    }
    pub fn after0(&self, origin: *const u8) -> bool {
        self.0.after0(origin)
    }
    pub fn after1(&self, origin: *const u8) -> bool {
        self.0.after1(origin)
    }
    pub fn to_str(&self) -> &str {
        self.0.to_str()
    }
    pub fn elf_base_from(&self, origin: *const u8) -> isize {
        self.0.elf_base_from(origin)
    }
    pub fn str_ptr(&self) -> String {
        format!("str_ptr: {:x?}", self.0)
    }
    pub fn str_ref(&self) -> &RawStrRef {
        &self.0
    }
}
