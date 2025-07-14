#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Process {
    pub pid: i64,
    pub state: i64,
    pub sp: *mut crate::constants::VaddrT,
    pub page_table: crate::constants::PaddrT,
    pub stack: [u8; 8192],
}

impl Process {
    pub const fn new() -> Self {
        Process {
            pid: 0,
            state: crate::constants::PROC_UNUSED,
            sp: core::ptr::null_mut(),
            page_table: 0,
            stack: [0; 8192],
        }
    }
}

static mut PROCS: [Process; crate::constants::PROCS_MAX] =
    [Process::new(); crate::constants::PROCS_MAX];
pub static mut IDLE_PROC: *mut Process = core::ptr::null_mut();
pub static mut CURRENT_PROC: *mut Process = core::ptr::null_mut();

#[unsafe(naked)]
#[no_mangle]
pub unsafe extern "C" fn switch_context(prev_sp: &*mut u32, next_sp: &*mut u32) {
    core::arch::naked_asm!(
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
    );
}

pub unsafe fn create_process(pc: u32) -> *mut Process {
    let mut proc = core::ptr::null_mut();
    let mut i = 0;

    for tmp in 0..crate::constants::PROCS_MAX {
        if PROCS[tmp].state == crate::constants::PROC_UNUSED {
            i = tmp;
            proc = &mut PROCS[i] as *mut Process;
            break;
        }
    }

    if !proc.is_null() {
        let sp = (&mut (*proc).stack as *mut [u8] as *mut u8)
            .add(core::mem::size_of_val(&(*proc).stack)) as *mut u32;
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

        let page_table = crate::paging::alloc_pages(1);
        let mut paddr =
            core::ptr::addr_of!(crate::__kernel_base) as *const u8 as crate::constants::PaddrT;
        while paddr
            < core::ptr::addr_of!(crate::__free_ram_end) as *const u8 as crate::constants::PaddrT
        {
            crate::paging::map_page(
                page_table,
                paddr,
                paddr,
                crate::constants::PAGE_R | crate::constants::PAGE_W | crate::constants::PAGE_X,
            );
            paddr += crate::constants::PAGE_SIZE as u32;
        }

        (*proc).pid = i as i64 + 1;
        (*proc).state = crate::constants::PROC_READY;
        (*proc).sp = sp.offset(-52);
        (*proc).page_table = page_table;
        proc
    } else {
        panic!("no free process slot");
    }
}

pub unsafe fn yield_proc() {
    let mut next = IDLE_PROC;
    for i in 0..crate::constants::PROCS_MAX {
        let proc = &mut PROCS[(CURRENT_PROC.as_ref().unwrap().pid as usize).wrapping_add(i)
            % crate::constants::PROCS_MAX] as *mut Process;

        if (*proc).state == crate::constants::PROC_READY && (*proc).pid > 0 {
            next = proc;
            break;
        }
    }

    if next == CURRENT_PROC {
        return;
    }

    let prev = CURRENT_PROC;
    CURRENT_PROC = next;

    core::arch::asm!(
        "sfence.vma",
        "csrw satp, {satp}",
        satp = in(reg) (((*next).page_table / crate::constants::PAGE_SIZE as u32) | crate::constants::SATP_SV32 as u32),
    );

    crate::write_csr!(
        "sscratch",
        (&mut (*next).stack as *mut [u8] as *mut u8).add(core::mem::size_of_val(&(*next).stack))
            as *mut u32
    );
    switch_context(&mut (*prev).sp, &(*next).sp)
}
