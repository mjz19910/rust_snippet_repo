use std::any::Any;
use std::ptr::addr_of;

use crate::{disabled, main};

use super::{
    debug_str_ref, elf_base, get_location, get_str_ref, get_type, is_cached_offset,
    iter_find_next_object, mark_offset_hit,
    metadata::{GetX, XVTable},
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
        let value = get_location(self.fns_arr);
        self.ptr_base = value.elf_base_from(self.elf_base_ptr);
        self.cur_offset = self.ptr_base - self.start_count[0];
        if value.before(self.elf_base_ptr) {
            return LoopBreak;
        }
        disabled!(println!("{} loop_iter: {:x?}", p_dbg(self), value));
        if value.after(self.elf_base_ptr) && value.after1(self.elf_base_ptr as usize) {
            disabled!(println!("{} {}", p_dbg(self), value.str_ptr()));
            add(&mut self.fns_arr, 1);
            return LoopContinue;
        }
        if value.before(self.last_func_ptr) {
            let opt = is_cached_offset(self);
            mark_offset_hit(self, opt);
            const N: usize = 3;
            let value: XVTable<(), N> = get_type(self.fns_arr);
            let vtable_rva: [isize; N] = value.vtable_fns.map(|x| elf_base(self.elf_base_ptr, x));
            let _vtable_num: [usize; N] = value.vtable_fns.map(|x| x as usize);
            let mut get_x: Option<Box<dyn GetX>> = Some(Box::new(value));
            let result = iter_find_next_object(self, &mut get_x);
            disabled!(println!(
                "{} print_get_x_box: {}",
                p_dbg(self),
                get_x.expect("get_x is some").x_value()
            ));
            if let LoopContinue = result {
                return result;
            }
            print!("state_check_3: {} {:#x}: ", p_dbg(self), self.cur_offset);
            print!("({:x?}) ", vtable_rva);
            print!("@!(3) ");
            print!("{:x?}", value);
            println!();
            add(&mut self.fns_arr, 6);
            return LoopBreak;
        }
        if value.is_small() {
            if let Some(str_v) = value.to_str() {
                disabled!(value.debug(self, str_v));
            }
            add(&mut self.fns_arr, 3);
            return LoopContinue;
        }
        if self.cur_offset > 0x1000 {
            let value = get_str_ref(self.fns_arr);
            if let Some(str_v) = value.to_str() {
                disabled!(debug_str_ref(self, str_v, value));
            }
            add(&mut self.fns_arr, 2);
            return LoopContinue;
        }
        println!("loop_inner_1(break): {} {:x?}", p_dbg(self), value);
        LoopBreak
    }
}
