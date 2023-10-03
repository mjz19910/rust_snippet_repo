use core::ptr;
use std::mem;
use std::slice;

#[no_mangle]
extern "C" fn gdb_bp() {
    println!("hit gdb_bp");
}

/// # Safety
///
/// This function will panic when you read null ptr
pub fn read_as_optional<T: Copy>(value: *const T) -> Option<T> {
    if value as usize != 0 {
        Some(unsafe { *value })
    } else {
        None
    }
}

pub fn lambda_ref() {
    println!("[lambda_ref]");
    let lambda_a = 0u64;
    let lambda_b = (|| ()) as fn();
    let lambda_x = 0x4151u64;
    let lambda_z = 0u64;
    let lambda = || (lambda_a, lambda_b, lambda_x, lambda_z);
    let c = ptr::addr_of!(lambda);
    let a = c as *const *const u64;
    let size = mem::size_of_val(&lambda);
    let slice_len = size / 8;
    let lambda_parts = unsafe { slice::from_raw_parts(a, slice_len) };
    println!(
        "addr of lambda info: {:x?} {:#x} {:x?}",
        c, size, lambda_parts
    );
    println!("read all captured refs: {:x?}", unsafe {
        (
            slice::from_raw_parts(lambda_parts[0], 1),
            slice::from_raw_parts(lambda_parts[1], 1),
            slice::from_raw_parts(lambda_parts[2], 1),
            slice::from_raw_parts(lambda_parts[3], 1),
        )
    });
    let func = gdb_bp as extern "C" fn();
    let func_size = mem::size_of_val(&func);
    let f_ptr = ptr::addr_of!(func);
    let ptr = f_ptr as *const u64;
    println!("f_ptr: {:#x?}[{:#x}]", unsafe { *ptr }, func_size);
    let func = read_as_optional(f_ptr).unwrap();
    func();
    let value = unsafe { *(lambda_parts[1] as *const usize) };
    match value {
        0 => println!("[null]"),
        _ if value > 0x1000 && value < 0x7ffffffff000 => (),
        _ => println!("*lambda_parts[1]: [{:x?}]", value),
    }
    let ret = lambda();
    println!("(ret.1): {:x?}[{:#x?}]", ret.1, mem::size_of_val(&ret.1));
}
