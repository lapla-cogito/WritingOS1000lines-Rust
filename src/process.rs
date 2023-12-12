use crate::{constants::*, println};
use core::{arch::asm, ptr};

#[derive(Copy, Clone, Debug)]
pub struct Process {
    pub pid: i32,
    pub state: i32,
    pub sp: *mut u32,
    pub stack: [u8; 8192],
}

impl Process {
    pub const fn new() -> Self {
        Process {
            pid: 0,
            state: PROC_UNUSED,
            sp: ptr::null_mut(),
            stack: [0; 8192],
        }
    }
}

static mut PROCS: [Process; PROCS_MAX] = [Process::new(); PROCS_MAX];

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

pub unsafe fn create_process(pc: usize) -> Process {
    let mut proc: Option<Process> = None;
    let mut i = 0;

    for tmp in 0..PROCS_MAX {
        if PROCS[tmp].state == PROC_UNUSED {
            i = tmp;
            proc = Some(PROCS[i]);
            break;
        }
    }

    if proc.is_some() {
        let slice = &PROCS[i].stack;
        let sp = slice.as_ptr().offset(8192) as *mut u32;
        unsafe {
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
            sp.offset(-52).write(pc as u32); // ra
        }

        PROCS[i].pid = i as i32 + 1;
        PROCS[i].state = PROC_READY;
        PROCS[i].sp = sp.offset(-52);
        PROCS[i]
    } else {
        println!("no free process slot");
        loop {}
    }
}
