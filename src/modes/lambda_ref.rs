use core::ptr;
use std::collections::VecDeque;
use std::mem;
use std::slice;

#[no_mangle]
extern "C" fn gdb_bp() {
    println!("hit gdb_bp");
}

pub fn read_as_optional<T: Copy>(value: *const T) -> Option<T> {
    if value as usize != 0 {
        Some(unsafe { *value })
    } else {
        None
    }
}

fn show_val_1<T>(value: T) -> Vec<u64> {
    let c = ptr::addr_of!(value);
    let a = c as *const u64;
    let size = mem::size_of_val(&value);
    let lambda_parts = unsafe { slice::from_raw_parts(a, size / 8) };
    lambda_parts.to_owned()
}
fn show_val_2<'a, T: ?Sized>(value: &'a T, sizes: VecDeque<usize>) -> Vec<&'a [u64]> {
    let mut sizes = sizes.clone();
    let c = ptr::addr_of!(value);
    let a = c as *const *const u64;
    let size = mem::size_of_val(&value);
    let slice_len = size / 8;
    let lambda_parts = unsafe { slice::from_raw_parts(a, slice_len) };
    let mut ret_parts = vec![];
    for item in lambda_parts {
        let size = sizes.pop_front().unwrap();
        let slice = unsafe { slice::from_raw_parts(*item, size) };
        ret_parts.push(slice);
    }
    ret_parts
}

pub fn lambda_ref() {
    println!("[lambda_ref]");
    let lambda_a = 0u64;
    let lambda_b = (|| ()) as fn();
    let lambda_x = 0x4151u64;
    let lambda_z = 0u64;
    let lambda = || (lambda_a, lambda_b, lambda_x, lambda_z);
    let fn_ptr = &lambda as &dyn FnOnce() -> (u64, fn(), u64, u64);
    let fn_ptr_data_level1 = show_val_1(fn_ptr);
    println!("fn_ptr_data_level1={fn_ptr_data_level1:x?}");
    let fn_ptr_data_level2 = show_val_2(fn_ptr, VecDeque::from([4, 1]));
    println!("fn_ptr_data_level2={fn_ptr_data_level2:x?}");
    let gdb_bp_fn = gdb_bp as extern "C" fn();
    let func_size = mem::size_of_val(&gdb_bp_fn);
    let gdb_bp_ptr = ptr::addr_of!(gdb_bp_fn);
    let gdb_bp_ptr_u64 = gdb_bp_ptr as *const u64;
    println!(
        "f_ptr: {:#x?}[{:#x}]",
        unsafe { *gdb_bp_ptr_u64 },
        func_size
    );
    let gdb_bp_fn = read_as_optional(gdb_bp_ptr).unwrap();
    gdb_bp_fn();
    let (_ret_a, ret_b, _ret_x, _ret_z) = lambda();
    println!("(ret.1): {:x?}[{:#x?}]", ret_b, mem::size_of_val(&ret_b));
}
