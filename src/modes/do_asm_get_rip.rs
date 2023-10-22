use std::arch::asm;

#[inline(never)]
pub fn asm_get_rip() -> u64 {
    let mut value: u64;
    unsafe {
        asm!("call 1f", "1:", "pop rax", out("rax") value);
    }
    value
}

#[inline(never)]
pub fn do_asm_get_rip() {
    let ptr = &(asm_get_rip as fn() -> _) as &fn() -> _ as *const fn() -> _;
    let ptr_1 = ptr as *const fn() -> u64;
    println!("fn_ptr  : {:#x}", asm_get_rip as fn() -> _ as usize);
    let rip = unsafe { *ptr_1 }();
    println!("from_asm: {rip:#x}");
    println!("asm_code: {:x?}", unsafe { *ptr.cast::<&[u8; 9]>() });
}
