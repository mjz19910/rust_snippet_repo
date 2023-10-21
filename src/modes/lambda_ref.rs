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

fn get_size<'a, T: ?Sized>(a: &'a T) -> usize {
    size_of_val(&a) / 8
}

fn show_val_1<'a, T: ?Sized, U: Clone>(value: &'a T) -> Vec<U> {
    let data = addr_of!(value).cast::<U>();
    let lambda_parts = unsafe { from_raw_parts(data, get_size(value)) };
    lambda_parts.to_vec()
}

fn show_val_2<'a, T: (FnOnce() -> V) + ?Sized, V: Copy>(
    value: &'a T,
    mut sizes: VecDeque<usize>,
) -> (V, Vec<&'a [u64]>) {
    let data = addr_of!(value).cast::<*const u64>();
    let lambda_parts = unsafe { from_raw_parts(data, get_size(value)) };
    let mut ret_parts = vec![];
    let mut rp = |data, len| {
        ret_parts.push(unsafe { from_raw_parts(data, len) });
    };
    rp(lambda_parts[0], size_of::<V>() / 8);
    rp(lambda_parts[1], sizes.pop_front().unwrap());
    assert_eq!(get_size(value), 2);
    let a1 = ret_parts[0] as *const [u64];
    let a3 = a1 as *const [*const ()];
    let a4 = unsafe { &*a3 };
    let mut a2 = vec![];
    for &item in a4 {
        let i2 = item.cast::<u64>();
        a2.push(unsafe { *i2 });
    }
    let v1 = a2.as_ptr().cast::<V>();
    let v2 = unsafe { *v1 };
    let mut ret_parts2 = vec![];
    for &item in &ret_parts[1..] {
        ret_parts2.push(item);
    }
    (v2, ret_parts2)
}

#[derive(Debug, Clone, Copy)]
pub struct LambdaType<'a> {
    pub captures: &'a LambdaTypeCaptures<'a>,
    pub metadata: &'static LambdaTypeMeta,
}
#[derive(Debug)]
pub struct LambdaTypeMeta {
    pub drop_in_place: u64,
    pub size_of: u64,
    pub align_of: u64,
    pub call_once: u64,
    pub closure: u64
}
#[derive(Debug)]
pub struct LambdaTypeCaptures<'a> {
    pub a: &'a u64,
    pub b: &'a fn(),
    pub x: &'a u64,
    pub z: &'a u64
}

pub fn lambda_ref() {
    println!("[lambda_ref]");
    let lambda_a = 0u64;
    let lambda_b = (|| ()) as fn();
    let lambda_x = 0x4151u64;
    let lambda_z = 0u64;
    let lambda = || (lambda_a, lambda_b, lambda_x, lambda_z);
    let fn_ptr = &lambda as &dyn Fn() -> _;
    let fn_ptr_data_level1: Vec<u64> = show_val_1(fn_ptr);
    println!("fn_ptr_data_level1={fn_ptr_data_level1:x?}");
    let (fn_ptr_data_level2, fn_ptr_data_vec2) = show_val_2(fn_ptr, VecDeque::from([4]));
    println!("fn_ptr_data2.0={fn_ptr_data_level2:x?}");
    println!("fn_ptr_data2.1={fn_ptr_data_vec2:x?}");
    let fn_info1 = addr_of!(fn_ptr) as *const LambdaType;
    let fn_info_p = unsafe { *fn_info1 };
    println!("fn_info_p={fn_info_p:x?}");
    let gdb_bp_fn = gdb_bp as extern "C" fn();
    assert_eq!(size_of_val(&gdb_bp_fn), 8);
    let gdb_bp_ptr = addr_of!(gdb_bp_fn);
    let u64_ptr = gdb_bp_ptr.cast::<u64>();
    println!("gdb_bp_fn: {:#x?}", unsafe { *u64_ptr });
    read_as_optional(gdb_bp_ptr).unwrap()();
    let (_ret_a, ret_b, _ret_x, _ret_z, ..) = lambda();
    assert_eq!(size_of_val(&ret_b), 8);
    println!("ret_b: {ret_b:x?}");
}
