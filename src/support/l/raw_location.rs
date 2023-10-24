use super::{PtrIter, RawStrRef};

#[derive(Clone, Copy, Debug)]
pub struct RawLocation(RawStrRef, u32, u32);
impl RawLocation {
    pub fn debug(&self, state: &PtrIter, str_v: &str) {
        println!(
            "{} debug_location_value: ({:#x}, {:?}, {:#05x}, {:#04x})",
            state.p_dbg(),
            self.elf_base_from(state.elf_origin),
            str_v,
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
    pub fn before0(&self, ptr: *const u8) -> bool {
        self.0.before0(ptr)
    }
    pub fn after0(&self, ptr: *const u8) -> bool {
        self.0.after0(ptr)
    }
    pub fn after1(&self, ptr: usize) -> bool {
        self.0.after1(ptr)
    }
    pub fn to_str(&self) -> Option<&str> {
        self.0.as_os_str().to_str()
    }
    pub fn elf_base_from(&self, ptr: *const u8) -> isize {
        self.0.elf_base_from(ptr)
    }
    pub fn str_ptr(&self) -> String {
        format!("str_ptr: {:x?}", self.0)
    }
}
