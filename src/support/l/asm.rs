use std::arch::asm;

pub fn jmp_loop() {
    unsafe { asm!("2:", "jmp 2b") }
}
