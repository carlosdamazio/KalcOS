use core::arch::asm;

pub fn idle() -> () {
    unsafe { asm!("hlt") }
}
