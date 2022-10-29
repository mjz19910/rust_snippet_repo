use core::ptr;
use std::mem;
use std::slice;

pub fn lambda_moved() {
    println!("[lambda_moved]");
    let a = 0u64;
    let b = || ();
    let x = 0x4151u64;
    let z = 0u64;
    let lambda = move || (a, b, x, z);
    let ptr = ptr::addr_of!(lambda);
    let size = mem::size_of_val(&lambda);
    println!(
        "{:x?}",
        (ptr, size, unsafe {
            slice::from_raw_parts(ptr as *const usize, size / 8)
        },)
    );
}
