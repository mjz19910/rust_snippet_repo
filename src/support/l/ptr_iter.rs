use std::any::Any;
use std::ptr::{addr_of, metadata};
use std::usize;

use crate::{disabled, main};

use super::{
    elf_base, get_location, get_type, iter_find_next_object,
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

macro_rules! sp1 {
    ($a:expr, $p:expr, $n:expr) => {
        sub(&mut $a, $n);
        $p += $n;
    };
}

#[derive(Debug)]
pub struct PtrIter {
    pub fns_arr: *const *const (),
    pub elf_origin: *const u8,
    pub last_func_ptr: *const u8,
    pub main_rva: isize,
    pub ptr_base: isize,
    pub start_count: isize,
    pub is_debug_build: bool,
}
impl PtrIter {
    pub fn new() -> Result<Self, String> {
        let value = 0;
        let ptr_metadata = metadata::<dyn Any>(&value);
        let vtable = get_vtable::<dyn Any, 1>(&ptr_metadata);
        let fns_arr: *const *const () = addr_of!(vtable.drop_in_place).cast();
        let symbol_info = vtable
            .drop_in_place
            .symbol_info()
            .ok_or_else(|| "drop_in_place.symbol_info() is None")?;
        let elf_origin = get_dli_fbase(symbol_info)
            .ok_or_else(|| "symbol_info.dli_fbase is None")?
            .cast();
        let last_func_ptr = _fini as *const u8;
        let main_rva = elf_base(elf_origin, main as *const u8);
        let is_debug_build = main_rva > 0x18000;
        Ok(Self {
            fns_arr,
            elf_origin,
            last_func_ptr,
            main_rva,
            ptr_base: 0,
            start_count: 0,
            is_debug_build,
        })
    }
    pub fn process_one(&mut self) -> LoopState {
        let value = get_location(self.fns_arr);
        self.ptr_base = value.elf_base_from(self.elf_origin);
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
            let value: XVTable<(), 3> = get_type(self.fns_arr);
            let vtable_rva = value.vtable_fns.map(|x| elf_base(self.elf_origin, x));
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
            print!("state_check_3: {} {:#x}: ", self.p_dbg(), self.ptr_base);
            print!("({:x?}) ", vtable_rva);
            print!("@!(3) ");
            print!("{:x?}", value);
            println!();
            add(&mut self.fns_arr, 6);
            return LoopBreak;
        }
        if value.is_small() {
            if let Some(str_v) = value.to_str() {
                value.debug(self, str_v);
            }
            add(&mut self.fns_arr, 3);
            return LoopContinue;
        }
        if self.ptr_base > 0 {
            if let Some(str_v) = value.to_str() {
                value.str_ref().debug(self, str_v);
            }
            add(&mut self.fns_arr, 2);
            return LoopContinue;
        }
        println!("process_one(break): {} {:x?}", self.p_dbg(), value);
        LoopBreak
    }
    fn offset_fns_arr(fns_arr: &mut *const *const (), ptr_count: &mut usize, n: usize) {
        sub(fns_arr, n);
        *ptr_count += n;
    }
    pub fn run(&mut self) -> Result<(), String> {
        let step_count = Rc::new(RefCell::new(0));
        let mut pos = self.fns_arr as usize;
        pos -= pos % 0x10;
        self.start_count = elf_base(self.elf_origin, pos as *const u8) - 0xf100000;
        disabled!(println!(
            "{} main_rva_ptr: {:#x?}",
            self.p_dbg(),
            self.main_rva
        ));
        let mut ptr_count = 0;
        let mut fns_arr = self.fns_arr;
        Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 7);
        // find_begin_ptrs
        if self.is_debug_build {
            Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 0x92);
        } else {
            Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 0xb8);
        }
        let mut loop_count = 0;
        loop {
            let value: [u64; 5] = get_type(fns_arr);
            match value {
                [2, 0, 0, 0, val] if val > 0x1000 => {
                    ptr_count -= 7;
                    break;
                }
                [0, 0, 0, 0, 0] => {
                    fns_arr = self.fns_arr;
                    ptr_count = 0;
                }
                _ => {
                    sp1!(fns_arr, ptr_count, 1);
                }
            }
            loop_count += 1;
        }
        if loop_count > 0 {
            println!(
                "{} find_begin_ptrs: sub({:#x}, {:#x?}, {:#x?})",
                self.p_dbg(),
                self.fns_arr as isize - fns_arr as isize - ((ptr_count + 7) * 8) as isize,
                ptr_count,
                loop_count,
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
        if self.is_debug_build {
            "D"
        } else {
            "R"
        }
    }
}
