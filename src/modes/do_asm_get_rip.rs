use std::arch::asm;

#[inline(never)]
pub fn asm_get_rip() -> usize {
    let mut value: usize;
    unsafe {
        asm!("call 1f", "1:", "pop {0}", out(reg) value, options(nostack));
    }
    value
}
trait RefAsPtr {
    fn as_ptr(&self) -> *const Self {
        self as *const _
    }
}
impl<T> RefAsPtr for T {}
#[inline(never)]
pub fn do_asm_get_rip() {
    let asm_ptr = asm_get_rip as fn() -> _;
    let ptr = RefAsPtr::as_ptr(&asm_ptr);
    println!("fn_ptr  : {:#x}", asm_ptr as usize);
    let fn_ = unsafe { *ptr };
    let rip = fn_();
    println!("from_asm: {rip:#x}");
    let v_ptr = unsafe { *ptr.cast::<&u8>() };
    if v_ptr == &0xe8 {
        println!("asm_code: {:x?}", unsafe { *ptr.cast::<&[u8; 7]>() });
    } else {
        println!("asm_code: {:x?}", unsafe {
            *ptr.cast::<&[u8; 1 + 6 + 8 + 2]>()
        });
    }
}
