use core::ptr::addr_of;
use std::collections::VecDeque;
use std::mem::{size_of, size_of_val};
use std::slice::from_raw_parts;

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

fn show_val_1<T, U: Clone>(value: T) -> Vec<U> {
    let data = addr_of!(value).cast::<U>();
    let lambda_parts = unsafe { from_raw_parts(data, size_of_val(&value) / 8) };
    lambda_parts.to_vec()
}
fn show_val_2<'a, T: (FnOnce() -> V) + ?Sized, V: Copy>(
    value: &'a T,
    sizes: VecDeque<usize>,
) -> (V, Vec<&'a [u64]>) {
    let mut sizes = sizes.clone();
    let c = addr_of!(value);
    let a = c.cast::<*const u64>();
    let size = size_of_val(&value);
    let slice_len = size / 8;
    let lambda_parts = unsafe { from_raw_parts(a, slice_len) };
    let mut ret_parts = vec![];
    let mut rp = |data, len| {
        ret_parts.push(unsafe { from_raw_parts(data, len) });
    };
    rp(lambda_parts[0], size_of::<V>() / 8);
    rp(lambda_parts[1], sizes.pop_front().unwrap());
    for &item in &lambda_parts[2..] {
        rp(item, sizes.pop_front().unwrap());
    }
    let a1 = ret_parts[0] as *const [u64];
    let a3 = a1 as *const [*const ()];
    let a4 = unsafe { &*a3 };
    let mut a2 = vec![];
    for &item in a4 {
        let i2 = item as *const u64;
        a2.push(unsafe { *i2 });
    }
    let v1 = a2.as_ptr().cast::<V>();
    let v2 = unsafe { *v1 };
    let iter = &ret_parts[1..];
    let mut ret_parts2 = vec![];
    for &item in iter {
        ret_parts2.push(item);
    }
    (v2, ret_parts2)
}

pub fn lambda_ref() {
    println!("[lambda_ref]");
    let lambda_a = 0u64;
    let lambda_b = (|| ()) as fn();
    let lambda_x = 0x4151u64;
    let lambda_z = 0u64;
    let lambda = || (lambda_a, lambda_b, lambda_x, lambda_z);
    let fn_ptr = &lambda as &dyn FnOnce() -> _;
    let fn_ptr_data_level1: Vec<u64> = show_val_1(fn_ptr);
    println!("fn_ptr_data_level1={fn_ptr_data_level1:x?}");
    let (fn_ptr_data_level2, fn_ptr_data_vec2) = show_val_2(fn_ptr, VecDeque::from([4]));
    println!("fn_ptr_data2.0={fn_ptr_data_level2:x?}");
    println!("fn_ptr_data2.1={fn_ptr_data_vec2:x?}");
    let gdb_bp_fn = gdb_bp as extern "C" fn();
    let func_size = size_of_val(&gdb_bp_fn);
    let gdb_bp_ptr = addr_of!(gdb_bp_fn);
    let gdb_bp_ptr_u64 = gdb_bp_ptr as *const u64;
    println!(
        "gdb_bp_fn: {:#x?}[{:#x}]",
        unsafe { *gdb_bp_ptr_u64 },
        func_size
    );
    let gdb_bp_fn = read_as_optional(gdb_bp_ptr).unwrap();
    gdb_bp_fn();
    let (_ret_a, ret_b, _ret_x, _ret_z) = lambda();
    println!("(ret.1): {:x?}[{:#x?}]", ret_b, size_of_val(&ret_b));
}
