#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

fn memset(dest: *mut u8, val: u8, count: usize) {
    for i in 0..count {
        unsafe {
            *dest.offset(i as isize) = val;
        }
    }
}

#[no_mangle]
fn kernel_main() {
    let bss_size = unsafe { &__bss_end as *const u8 as usize - &__bss as *const u8 as usize };
    unsafe { memset(&__bss as *const u8 as *mut u8, 0, bss_size) };
    loop {}
}


#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}\n
        j {kernel_main}\n",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    loop {}
}
