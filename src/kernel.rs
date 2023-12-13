#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(fn_align)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(offset_of)]

mod constants;
mod paging;
mod process;
mod sbi;
mod util;

use crate::{
    paging::NEXT_PADDR,
    process::{yield_proc, CURRENT_PROC, IDLE_PROC},
    util::*,
};
use core::{arch::asm, panic::PanicInfo, ptr::addr_of};
use process::create_process;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u32;
    static mut __free_ram: u8;
    static __free_ram_end: u8;
    static __kernel_base: u8;
}

fn proc_a_entry() {
    loop {
        let _res = putchar('A');
        unsafe {
            yield_proc();
            for _ in 0..10000000 {
                asm!("nop");
            }
        }
    }
}

fn proc_b_entry() {
    loop {
        let _res = putchar('B');
        unsafe {
            yield_proc();
            for _ in 0..10000000 {
                asm!("nop");
            }
        }
    }
}

#[no_mangle]
fn kernel_main() {
    unsafe {
        memset(
            &__bss as *const u8 as *mut u8,
            0,
            (__bss_end - __bss) as usize,
        );
        NEXT_PADDR = addr_of!(__free_ram) as u32;
        write_csr!("stvec", kernel_entry as usize);

        IDLE_PROC = create_process(0);
        (*IDLE_PROC).pid = -1;
        CURRENT_PROC = IDLE_PROC;

        create_process(proc_a_entry as u32);
        create_process(proc_b_entry as u32);
        yield_proc();
    }

    unreachable!();
}

#[no_mangle]
#[link_section = ".text.boot"]
pub unsafe extern "C" fn boot() -> ! {
    asm!(
        "mv sp, {stack_top}
        j {kernel_main}",
        stack_top = in(reg) &__stack_top,
        kernel_main = sym kernel_main,
    );
    loop {}
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn handle_trap(_frame: TrapFrame) {
    let mut scause: u32;
    let mut stval: u32;
    let mut sepc: u32;
    unsafe {
        asm!("csrr {}, scause", out(reg) scause);
        asm!("csrr {}, stval", out(reg) stval);
        asm!("csrr {}, sepc", out(reg) sepc);
    }
    panic!("scause: {:x}, stval: {:x}, sepc: {:x}", scause, stval, sepc);
}

#[no_mangle]
#[link_section = ".text.boot"]
#[repr(align(4))]
#[naked]
pub unsafe extern "C" fn kernel_entry() {
    asm!(
        "
        csrw sscratch, sp
        addi sp, sp, -4 * 31
        sw ra,  4 * 0(sp)
        sw gp,  4 * 1(sp)
        sw tp,  4 * 2(sp)
        sw t0,  4 * 3(sp)
        sw t1,  4 * 4(sp)
        sw t2,  4 * 5(sp)
        sw t3,  4 * 6(sp)
        sw t4,  4 * 7(sp)
        sw t5,  4 * 8(sp)
        sw t6,  4 * 9(sp)
        sw a0,  4 * 10(sp)
        sw a1,  4 * 11(sp)
        sw a2,  4 * 12(sp)
        sw a3,  4 * 13(sp)
        sw a4,  4 * 14(sp)
        sw a5,  4 * 15(sp)
        sw a6,  4 * 16(sp)
        sw a7,  4 * 17(sp)
        sw s0,  4 * 18(sp)
        sw s1,  4 * 19(sp)
        sw s2,  4 * 20(sp)
        sw s3,  4 * 21(sp)
        sw s4,  4 * 22(sp)
        sw s5,  4 * 23(sp)
        sw s6,  4 * 24(sp)
        sw s7,  4 * 25(sp)
        sw s8,  4 * 26(sp)
        sw s9,  4 * 27(sp)
        sw s10, 4 * 28(sp)
        sw s11, 4 * 29(sp)
        csrr a0, sscratch
        sw a0, 4 * 30(sp)
        mv a0, sp
        call handle_trap
        lw ra,  4 * 0(sp)
        lw gp,  4 * 1(sp)
        lw tp,  4 * 2(sp)
        lw t0,  4 * 3(sp)
        lw t1,  4 * 4(sp)
        lw t2,  4 * 5(sp)
        lw t3,  4 * 6(sp)
        lw t4,  4 * 7(sp)
        lw t5,  4 * 8(sp)
        lw t6,  4 * 9(sp)
        lw a0,  4 * 10(sp)
        lw a1,  4 * 11(sp)
        lw a2,  4 * 12(sp)
        lw a3,  4 * 13(sp)
        lw a4,  4 * 14(sp)
        lw a5,  4 * 15(sp)
        lw a6,  4 * 16(sp)
        lw a7,  4 * 17(sp)
        lw s0,  4 * 18(sp)
        lw s1,  4 * 19(sp)
        lw s2,  4 * 20(sp)
        lw s3,  4 * 21(sp)
        lw s4,  4 * 22(sp)
        lw s5,  4 * 23(sp)
        lw s6,  4 * 24(sp)
        lw s7,  4 * 25(sp)
        lw s8,  4 * 26(sp)
        lw s9,  4 * 27(sp)
        lw s10, 4 * 28(sp)
        lw s11, 4 * 29(sp)
        lw sp,  4 * 30(sp)
        sret
        ",
        options(noreturn)
    );
}
