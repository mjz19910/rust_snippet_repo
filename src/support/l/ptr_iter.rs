use crate::main;
use crate::support::metadata::XVTable;
use crate::support::symbol_info::get_dli_fbase;
use crate::support::symbol_info::SymbolInfo;
use std::any::Any;
use std::ptr::addr_of;

use super::elf_base;

extern "C" {
    fn _fini();
}

#[derive(Debug)]
pub struct PtrIter {
    pub fns_arr: *const *const (),
    pub start_count: [isize; 8],
    pub elf_base_ptr: *const u8,
    pub last_func_ptr: *const u8,
    pub main_rva: isize,
    pub cur_offset: isize,
    pub ptr_base: isize,
    pub is_debug_build: u8,
    pub runtime_code_gen_flag: bool,
}

impl PtrIter {
    pub fn new(vtable: &XVTable<dyn Any, 1>, runtime_code_gen_flag: bool) -> Self {
        let fns_arr: *const *const () = addr_of!(vtable.drop_in_place).cast();
        let info = vtable.drop_in_place.symbol_info();
        let elf_base_ptr = get_dli_fbase(info)
            .expect("get_dli_fbase on symbol_info is not None")
            .cast();
        let last_func_ptr = _fini as *const u8;
        let main_rva = elf_base(elf_base_ptr, main as *const u8);
        let is_debug_build = (main_rva > 0x18000).into();
        Self {
            fns_arr,
            start_count: [0; 8],
            elf_base_ptr,
            last_func_ptr,
            main_rva,
            cur_offset: 0,
            ptr_base: 0,
            is_debug_build,
            runtime_code_gen_flag,
        }
    }
}
