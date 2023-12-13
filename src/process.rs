use crate::{
    __free_ram_end, __kernel_base,
    constants::*,
    paging::{alloc_pages, map_page},
    println, write_csr,
};
use core::{arch::asm, mem, ptr};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Process {
    pub pid: i64,
    pub state: i64,
    pub sp: *mut VaddrT,
    pub page_table: PaddrT,
    pub stack: [u8; 8192],
}

impl Process {
    pub const fn new() -> Self {
        Process {
            pid: 0,
            state: PROC_UNUSED,
            sp: ptr::null_mut(),
            page_table: 0,
            stack: [0; 8192],
        }
    }
}

static mut PROCS: [Process; PROCS_MAX] = [Process::new(); PROCS_MAX];
pub static mut IDLE_PROC: *mut Process = ptr::null_mut();
pub static mut CURRENT_PROC: *mut Process = ptr::null_mut();

#[naked]
#[no_mangle]
pub unsafe extern "C" fn switch_context(prev_sp: &*mut u32, next_sp: &*mut u32) {
    asm!(
        "
        addi sp, sp, -13 * 4
        sw ra,  0  * 4(sp)
        sw s0,  1  * 4(sp)
        sw s1,  2  * 4(sp)
        sw s2,  3  * 4(sp)
        sw s3,  4  * 4(sp)
        sw s4,  5  * 4(sp)
        sw s5,  6  * 4(sp)
        sw s6,  7  * 4(sp)
        sw s7,  8  * 4(sp)
        sw s8,  9  * 4(sp)
        sw s9,  10 * 4(sp)
        sw s10, 11 * 4(sp)
        sw s11, 12 * 4(sp)
        sw sp, (a0)
        lw sp, (a1)
        lw ra,  0  * 4(sp)
        lw s0,  1  * 4(sp)
        lw s1,  2  * 4(sp)
        lw s2,  3  * 4(sp)
        lw s3,  4  * 4(sp)
        lw s4,  5  * 4(sp)
        lw s5,  6  * 4(sp)
        lw s6,  7  * 4(sp)
        lw s7,  8  * 4(sp)
        lw s8,  9  * 4(sp)
        lw s9,  10 * 4(sp)
        lw s10, 11 * 4(sp)
        lw s11, 12 * 4(sp)
        addi sp, sp, 13 * 4
        ret
        ",
        options(noreturn)
    );
}

pub unsafe fn create_process(pc: u32) -> *mut Process {
    let mut proc = ptr::null_mut();
    let mut i = 0;

    for tmp in 0..PROCS_MAX {
        if PROCS[tmp].state == PROC_UNUSED {
            i = tmp;
            proc = &mut PROCS[i] as *mut Process;
            break;
        }
    }

    if !proc.is_null() {
        let sp = (&mut (*proc).stack as *mut [u8] as *mut u8)
            .offset(mem::size_of_val(&(*proc).stack) as isize) as *mut u32;
        sp.offset(-4).write(0); // s11
        sp.offset(-8).write(0); // s10
        sp.offset(-12).write(0); // s9
        sp.offset(-16).write(0); // s8
        sp.offset(-20).write(0); // s7
        sp.offset(-24).write(0); // s6
        sp.offset(-28).write(0); // s5
        sp.offset(-32).write(0); // s4
        sp.offset(-36).write(0); // s3
        sp.offset(-40).write(0); // s2
        sp.offset(-44).write(0); // s1
        sp.offset(-48).write(0); // s0
        sp.offset(-52).write(pc); // ra

        let page_table = alloc_pages(1);
        let mut paddr = ptr::addr_of!(__kernel_base) as *const u8 as PaddrT;
        while paddr < ptr::addr_of!(__free_ram_end) as *const u8 as PaddrT {
            map_page(page_table, paddr, paddr, PAGE_R | PAGE_W | PAGE_X);
            paddr += PAGE_SIZE as u32;
        }

        (*proc).pid = i as i64 + 1;
        (*proc).state = PROC_READY;
        (*proc).sp = sp.offset(-52) as *mut u32;
        (*proc).page_table = page_table;
        proc
    } else {
        panic!("no free process slot");
    }
}

pub unsafe fn yield_proc() {
    let mut next = IDLE_PROC;
    for i in 0..PROCS_MAX {
        let proc = &mut PROCS
            [(CURRENT_PROC.as_ref().unwrap().pid as usize).wrapping_add(i) % PROCS_MAX as usize]
            as *mut Process;

        if (*proc).state == PROC_READY && (*proc).pid > 0 {
            next = proc;
            break;
        }
    }

    if next == CURRENT_PROC {
        return;
    }

    let prev = CURRENT_PROC;
    CURRENT_PROC = next;

    asm!(
        "sfence.vma",
        "csrw satp, {satp}",
        satp = in(reg) (((*next).page_table / PAGE_SIZE as u32) | SATP_SV32 as u32),
    );
    write_csr!(
        "sscratch",
        (&mut (*next).stack as *mut [u8] as *mut u8)
            .offset(mem::size_of_val(&(*next).stack) as isize) as *mut u32
    );
    switch_context(&mut (*prev).sp, &(*next).sp)
}
