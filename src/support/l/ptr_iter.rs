use std::any::Any;
use std::ptr::addr_of;

use crate::{disabled, main};

use super::{
    elf_base, get_type, loop_branch_1, loop_branch_2, loop_branch_3,
    metadata::XVTable,
    p_dbg,
    ptr_math::add,
    symbol_info::{get_dli_fbase, SymbolInfo},
    LoopState::{self, LoopBreak, LoopContinue},
};

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
    pub fn process_one(&mut self) -> LoopState {
        let value: (*const u8, usize, u32, u32) = get_type(self.fns_arr);
        self.ptr_base = elf_base(self.elf_base_ptr, value.0);
        self.cur_offset = self.ptr_base - self.start_count[0];
        if (value.0 as usize) < (self.elf_base_ptr as usize) {
            return LoopBreak;
        }
        disabled!(println!("{} loop_iter: {:x?}", p_dbg(self), value));
        if value.0 > self.elf_base_ptr && value.1 > (self.elf_base_ptr as usize) {
            disabled!(println!("{} str_ptr: {:x?}", p_dbg(self), value.0));
            add(&mut self.fns_arr, 1);
            return LoopContinue;
        }
        if value.0 < self.last_func_ptr {
            return loop_branch_3(self);
        }
        if value.3 < 0x1000 {
            return loop_branch_2(self);
        }
        if self.cur_offset > 0x1000 {
            return loop_branch_1(self);
        }
        println!("loop_inner_1(break): {} {:x?}", p_dbg(self), value);
        LoopBreak
    }
}
