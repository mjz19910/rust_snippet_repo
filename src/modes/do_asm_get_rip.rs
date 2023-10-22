use std::arch::asm;

#[inline(never)]
pub fn asm_get_rip() -> usize {
    let mut value: usize;
    unsafe {
        asm!("call 1f", "1:", "pop {0}", "lea {0},[{0}-6]", out(reg) value);
    }
    value
}

#[inline(never)]
pub fn do_asm_get_rip() {
    let ptr = &(asm_get_rip as fn() -> _) as &fn() -> _ as *const fn() -> _;
    println!("fn_ptr  : {:#x}", asm_get_rip as fn() -> _ as usize);
    let fn_ = unsafe { *ptr };
    let rip = fn_();
    assert_eq!(fn_ as usize, rip);
    println!("from_asm: {rip:#x}");
    println!("asm_code: {:x?}", unsafe { *ptr.cast::<&[u8; 13]>() });
}
