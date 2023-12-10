#![no_std]
#![no_main]

mod constants;
mod sbi;

use constants::*;
use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

unsafe fn sbi_call(
    mut arg0: usize,
    mut arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
    fid: usize,
    eid: usize,
) -> Result<usize, SbiErr> {
    asm!(
        "ecall",
        inout("a0") arg0 => arg0,
        inout("a1") arg1 => arg1,
        in("a2") arg2,
        in("a3") arg3,
        in("a4") arg4,
        in("a5") arg5,
        in("a6") fid,
        in("a7") eid as usize,
    );

    let err = arg0 as isize;
    if err == SBI_SUCCESS {
        Ok(arg1)
    } else {
        Err(err)
    }
}

fn putchar(c: char) -> Result<(), SbiErr> {
    unsafe {
        let _res = sbi_call(c as usize, 0, 0, 0, 0, 0, 1, 1)?;
    }
    Ok(())
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
    const HELLO: &[u8] = b"Hello, world!\n";
    for &c in HELLO {
        putchar(c as char).unwrap();
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
