use std::any::Any;
use std::ptr::{addr_of, metadata};

use crate::{disabled, main};

use super::{
    constants::CODE_GEN_ENABLED,
    debug_str_ref, elf_base, get_location, get_str_ref, get_type, is_cached_offset,
    iter_find_next_object, mark_offset_hit,
    metadata::{get_vtable, GetX, XVTable},
    ptr_math::add,
    symbol_info::{get_dli_fbase, SymbolInfo},
    LoopState::{self, LoopBreak, LoopContinue},
};
use crate::support::{iter_type, ptr_math::sub};
use std::{cell::RefCell, rc::Rc};

extern "C" {
    fn _fini();
}

#[derive(Debug)]
pub struct PtrIter {
    pub fns_arr: *const *const (),
    pub start_count: [isize; 8],
    pub elf_origin: *const u8,
    pub last_func_ptr: *const u8,
    pub main_rva: isize,
    pub cur_offset: isize,
    pub ptr_base: isize,
    pub is_debug_build: u8,
    pub runtime_code_gen_flag: bool,
}
impl PtrIter {
    pub fn new() -> Result<Self, String> {
        let value = 0;
        let ptr_metadata = metadata::<dyn Any>(&value);
        let vtable = get_vtable::<dyn Any, 1>(&ptr_metadata);
        let fns_arr: *const *const () = addr_of!(vtable.drop_in_place).cast();
        let info = vtable.drop_in_place.symbol_info();
        let elf_origin = get_dli_fbase(info)
            .ok_or_else(|| "get_dli_fbase on symbol_info is not None")?
            .cast();
        let last_func_ptr = _fini as *const u8;
        let main_rva = elf_base(elf_origin, main as *const u8);
        let is_debug_build = (main_rva > 0x18000).into();
        Ok(Self {
            fns_arr,
            start_count: [0; 8],
            elf_origin,
            last_func_ptr,
            main_rva,
            cur_offset: 0,
            ptr_base: 0,
            is_debug_build,
            runtime_code_gen_flag: unsafe { CODE_GEN_ENABLED },
        })
    }
    pub fn process_one(&mut self) -> LoopState {
        let value = get_location(self.fns_arr);
        self.ptr_base = value.elf_base_from(self.elf_origin);
        self.cur_offset = self.ptr_base - self.start_count[0];
        if value.before0(self.elf_origin) {
            return LoopBreak;
        }
        disabled!(println!("{} loop_iter: {:x?}", self.p_dbg(), value));
        if value.after0(self.elf_origin) && value.after1(self.elf_origin) {
            disabled!(println!("{} {}", self.p_dbg(), value.str_ptr()));
            add(&mut self.fns_arr, 1);
            return LoopContinue;
        }
        if value.before0(self.last_func_ptr) {
            let opt = is_cached_offset(self);
            mark_offset_hit(self, opt);
            const N: usize = 3;
            let value: XVTable<(), N> = get_type(self.fns_arr);
            let vtable_rva: [isize; N] = value.vtable_fns.map(|x| elf_base(self.elf_origin, x));
            let _vtable_num: [usize; N] = value.vtable_fns.map(|x| x as usize);
            let mut get_x: Option<Box<dyn GetX>> = Some(Box::new(value));
            let result = iter_find_next_object(self, &mut get_x);
            disabled!(println!(
                "{} print_get_x_box: {}",
                self.p_dbg(),
                get_x.expect("get_x is some").x_value()
            ));
            if let LoopContinue = result {
                return result;
            }
            print!("state_check_3: {} {:#x}: ", self.p_dbg(), self.cur_offset);
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
        println!("loop_inner_1(break): {} {:x?}", self.p_dbg(), value);
        LoopBreak
    }
    pub fn run(&mut self) -> Result<(), String> {
        let step_count = Rc::new(RefCell::new(0));
        let mut pos = self.fns_arr as usize;
        pos -= pos % 0x10;
        self.start_count[0] = elf_base(self.elf_origin, pos as *const u8) - 0xf100000;
        disabled!(println!(
            "{} main_rva_ptr: {:#x?}",
            self.p_dbg(),
            self.main_rva
        ));
        let mut ptr_count = 0;
        let mut fns_arr_cur = self.fns_arr;
        macro_rules! sp {
            ($a:expr, $p:expr, $n:expr) => {
                sub(&mut $a, $n);
                $p += $n;
            };
            (x $a:expr, $p:expr, $n:expr) => {
                let n = $n;
                let v = n / 8;
                sub(&mut $a, v);
                $p += v;
            };
        }
        sp!(fns_arr_cur, ptr_count, 7);
        if self.is_debug_build == 1 {
            sp!(x fns_arr_cur, ptr_count, 0x490);
        } else {
            sp!(x fns_arr_cur, ptr_count, 0x708);
        }
        let mut loop_count = 0;
        loop {
            let value: [u64; 5] = get_type(fns_arr_cur);
            match value {
                [2, 0, 0, 0, val] if val > 0x1000 => {
                    ptr_count -= 7;
                    break;
                }
                [0, 0, 0, 0, 0] => {
                    fns_arr_cur = self.fns_arr;
                    ptr_count = 0;
                }
                _ => {
                    sp!(fns_arr_cur, ptr_count, 1);
                }
            }
            loop_count += 1;
        }
        if loop_count > 0 {
            println!(
                "{} find_begin_ptrs: sub({:#x}, {:#x?}, {:#x?})",
                self.p_dbg(),
                self.fns_arr as isize - fns_arr_cur as isize - ((ptr_count + 7) * 8) as isize,
                ptr_count * 8,
                loop_count * 8,
            )
        };
        sub(&mut self.fns_arr, ptr_count);
        let start_offset = elf_base(self.elf_origin, self.fns_arr);
        disabled!(println!(
            "{} elf_start_base: {:?} + {:#x?} = {:#x?}",
            self.p_dbg(),
            self.elf_origin,
            start_offset,
            self.fns_arr
        ));
        let fns_arr_start = self.fns_arr as *const u8;
        while let LoopContinue = self.process_one() {}
        if false {
            let mul = if false { 46 } else { 1 };
            self.fns_arr = iter_type::<*const ()>(8, self, &step_count, 8 * mul);
        }
        disabled!(println!(
            "{} elf_end_base: {:?} + {:#x?} + {:#x?}",
            self.p_dbg(),
            self.elf_origin,
            start_offset,
            elf_base(fns_arr_start, self.fns_arr)
        ));
        Ok(())
    }
    pub fn p_dbg(&self) -> &'static str {
        if self.is_debug_build == 1 {
            "D"
        } else {
            "R"
        }
    }
}
