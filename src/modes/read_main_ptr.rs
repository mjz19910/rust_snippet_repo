use std::slice;

pub fn read_main_ptr<T>(main_addr: fn() -> T) {
    println!("[read_main_ptr]");
    let fn_val = [main_addr];
    let fn_ref = &fn_val;
    let fn_ptr = fn_ref.as_ptr() as *const fn();
    let ptr = unsafe {
        let ptr2 = (*fn_ptr) as *const usize;
        (
            ptr2 as usize,
            (fn_ptr as *const usize) as usize,
            slice::from_raw_parts(ptr2 as *const u8, 12),
        )
    };
    println!("{:#x} -> {:#x}", ptr.0, ptr.1);
    println!("{:#x} -> {:x?}", ptr.1, ptr.2);
}
