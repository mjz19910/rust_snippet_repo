use crate::{
    disabled,
    support::{
        constants::CODE_GEN_ENABLED, elf_base, get_type, iter_type, metadata::get_vtable, p_dbg,
        ptr_math::sub, LoopState::LoopContinue, PtrIter,
    },
};
use std::{any::Any, cell::RefCell, ptr::metadata, rc::Rc};

pub fn ptr_meta_run() -> Result<(), String> {
    let mut runtime_code_gen_flag = false;
    if unsafe { CODE_GEN_ENABLED } {
        runtime_code_gen_flag = true;
    }
    let value = 0;
    let ptr_metadata = metadata::<dyn Any>(&value);
    let vtable = get_vtable(&ptr_metadata);
    let step_count = Rc::new(RefCell::new(0));
    let mut state = PtrIter::new(vtable, runtime_code_gen_flag);
    let mut pos = state.fns_arr as usize;
    pos -= pos % 0x10;
    state.start_count[0] = elf_base(state.elf_base_ptr, pos as *const u8) - 0xf100000;
    disabled!(println!(
        "{} main_rva_ptr: {:#x?}",
        p_dbg(&state),
        state.main_rva
    ));
    let mut ptr_count = 0;
    let mut fns_arr_cur = state.fns_arr;
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
    if state.is_debug_build == 1 {
        sp!(x fns_arr_cur, ptr_count, 0x490);
    } else {
        sp!(x fns_arr_cur, ptr_count, 0x5c8);
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
                fns_arr_cur = state.fns_arr;
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
            p_dbg(&state),
            state.fns_arr as isize - fns_arr_cur as isize - ((ptr_count + 7) * 8) as isize,
            ptr_count * 8,
            loop_count * 8,
        )
    };
    sub(&mut state.fns_arr, ptr_count);
    let start_offset = elf_base(state.elf_base_ptr, state.fns_arr);
    disabled!(println!(
        "{} elf_start_base: {:?} + {:#x?} = {:#x?}",
        p_dbg(&state),
        state.elf_base_ptr,
        start_offset,
        state.fns_arr
    ));
    let fns_arr_start = state.fns_arr as *const u8;
    while let LoopContinue = state.process_one() {}
    if false {
        let mul = if false { 46 } else { 1 };
        state.fns_arr = iter_type::<*const ()>(8, &state, &step_count, 8 * mul);
    }
    disabled!(println!(
        "{} elf_end_base: {:?} + {:#x?} + {:#x?}",
        p_dbg(&state),
        state.elf_base_ptr,
        start_offset,
        elf_base(fns_arr_start, state.fns_arr)
    ));
    Ok(())
}
