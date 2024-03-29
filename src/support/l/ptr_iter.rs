use super::{
    elf_base, get_location, get_type, iter_find_next_object,
    metadata::{get_vtable, GetX, XVTable},
    ptr_math::{add, sub},
    symbol_info::{get_dli_fbase, SymbolInfo},
    LoopState::{self, LoopBreak, LoopContinue},
};
use crate::{disabled, main};
use std::any::Any;
use std::ptr::{addr_of, metadata};
use std::usize;
extern "C" {
    fn _fini();
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
            .ok_or("drop_in_place.symbol_info() is None")?;
        let elf_origin = get_dli_fbase(symbol_info)
            .ok_or("symbol_info.dli_fbase is None")?
            .cast();
        let last_func_ptr = _fini as *const u8;
        let main_rva = unsafe { elf_base(elf_origin, main as *const u8) };
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
        use crate::support::get_debug_flag_state;
        let value = get_location(self.fns_arr);
        self.ptr_base = unsafe { value.elf_base_from(self.elf_origin) };
        if value.before0(self.elf_origin) {
            for _ in 0..7 {
                let v: u64 = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, 1);
            }
            {
                let v: [u64; 2] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, 2);
            }
            for _ in 0..5 {
                let v: [u64; 2] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done3: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, 2);
            }
            for _ in 0..2 {
                let v = get_location(self.fns_arr);
                let v = v.str_ref();
                if get_debug_flag_state() {
                    println!("{} before_done4: {:x?} {}", self.p_dbg(), v, v.to_str());
                }
                add(&mut self.fns_arr, 2);
            }
            {
                const N: usize = 6;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done5: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done6: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            for _ in 0..12 {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done7: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                let v: u64 = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done8: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, 1);
            }
            {
                const N: usize = 10;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done9: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 3;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.1: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            if self.is_debug_build {
                const N: usize = 24;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.2: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            } else {
                const N: usize = 25;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.2: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            for _ in 0..8 {
                const N: usize = 11;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.3: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            if self.is_debug_build {
                const N: usize = 3;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.4: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            } else {
                const N: usize = 5;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.4: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            for _ in 0..9 {
                const N: usize = 11;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.5: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            if self.is_debug_build {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.6: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            } else {
                const N: usize = 3;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.7: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            for _ in 0..2 {
                const N: usize = 11;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.8: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 6;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done1.9: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.1: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.2: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 5;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.3: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.4: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 11;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.5: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 3;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.6: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 3;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.7: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.8: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            {
                const N: usize = 1;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done2.9: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            let v: [u64; 13] = get_type(self.fns_arr);
            assert_eq!(v[0..3], [0x1, 0x1000, 0]);
            assert_eq!(v[3..], [0; 10]);
            {
                const N: usize = 2;
                let v: [u64; N] = get_type(self.fns_arr);
                if get_debug_flag_state() {
                    println!("{} before_done3.1: {:x?}", self.p_dbg(), v);
                }
                add(&mut self.fns_arr, N);
            }
            return LoopBreak;
        }
        if value.after0(self.elf_origin) && value.after1(self.elf_origin) {
            if get_debug_flag_state() {
                let v = value.as_ptr();
                println!("{} str_ptr: {:x?}", self.p_dbg(), v);
            }
            add(&mut self.fns_arr, 1);
            return LoopContinue;
        }
        if value.before0(self.last_func_ptr) {
            let value: XVTable<(), 3> = get_type(self.fns_arr);
            let vtable_rva = value
                .vtable_fns
                .map(|x| unsafe { elf_base(self.elf_origin, x) });
            let mut get_x: Box<dyn GetX> = <dyn GetX>::default_box();
            let result = iter_find_next_object(self, &mut get_x);
            disabled!(println!(
                "{} print_get_x_box: {}",
                self.p_dbg(),
                get_x.x_value()
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
            value.debug(self);
            add(&mut self.fns_arr, 3);
            return LoopContinue;
        }
        if self.ptr_base > 0 {
            value.str_ref().debug(self);
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
        let mut pos = self.fns_arr as usize;
        pos -= pos % 0x10;
        self.start_count = unsafe { elf_base(self.elf_origin, pos as *const u8) } - 0xf100000;
        disabled!(println!(
            "{} main_rva_ptr: {:#x?}",
            self.p_dbg(),
            self.main_rva
        ));
        let mut ptr_count = 0;
        let mut fns_arr = self.fns_arr;
        Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 4);
        // find_begin_ptrs
        if self.is_debug_build {
            Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 0x95);
        } else {
            Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 0x2b);
        }
        let mut loop_count = 0;
        loop {
            let value: [u64; 5] = get_type(fns_arr);
            match value {
                [2, 0, 0, 0, val] if val > 0x1000 => {
                    ptr_count -= 4;
                    break;
                }
                [0, 0, 0, 0, 0] => {
                    fns_arr = self.fns_arr;
                    ptr_count = 0;
                }
                _ => {
                    Self::offset_fns_arr(&mut fns_arr, &mut ptr_count, 1);
                }
            }
            loop_count += 1;
        }
        if loop_count > 0 {
            println!(
                "{} find_begin_ptrs: sub({:#x}, {:#x?}, {:#x?})",
                self.p_dbg(),
                self.fns_arr as isize - fns_arr as isize - ((ptr_count - 4) * 8) as isize,
                ptr_count,
                loop_count,
            )
        };
        sub(&mut self.fns_arr, ptr_count);
        let start_offset = unsafe { elf_base(self.elf_origin, self.fns_arr) };
        disabled!(println!(
            "{} elf_start_base: {:?} + {:#x?} = {:#x?}",
            self.p_dbg(),
            self.elf_origin,
            start_offset,
            self.fns_arr
        ));
        let fns_arr_start = self.fns_arr as *const u8;
        while let LoopContinue = self.process_one() {}
        disabled!(println!(
            "{} elf_end_base: {:?} + {:#x?} + {:#x?}",
            self.p_dbg(),
            self.elf_origin,
            start_offset,
            unsafe { elf_base(fns_arr_start, self.fns_arr) }
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
