#![no_std]
#![no_main]
#![feature(c_variadic)]

mod constants;
mod sbi;
mod util;

use crate::util::*;
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut buf = [0u8; 64];
    let _s: &str =
        write_to::show(&mut buf, format_args!("panicked at: {:?}", info.location())).unwrap();
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

#[no_mangle]
fn kernel_main() {
    if strcmp("aa".as_ptr(), "aa".as_ptr()) == 0 {
        let _res = putchar('a');
    } else {
        let _res = putchar('b');
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
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
