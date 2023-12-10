use crate::constants::*;
use core::{arch::asm, ffi::c_void};

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

pub fn putchar(c: char) -> Result<(), SbiErr> {
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

pub fn memcpy(dst: *mut c_void, src: *const c_void, n: SizeT) {
    unsafe {
        let mut p_dst = dst as *mut u8;
        let mut p_src = src as *const u8;

        for _ in 0..n {
            *p_dst = *p_src;
            p_dst = p_dst.add(1);
            p_src = p_src.add(1);
        }
    }
}

pub fn strcpy(dst: *mut i8, src: *const i8) -> *mut i8 {
    unsafe {
        let mut p_dst = dst;
        let mut p_src = src;

        while *p_src != 0 {
            *p_dst = *p_src;
            p_dst = p_dst.add(1);
            p_src = p_src.add(1);
        }

        *p_dst = 0;

        dst
    }
}

pub fn strcmp(s1: *const u8, s2: *const u8) -> i32 {
    unsafe {
        let mut p_s1 = s1;
        let mut p_s2 = s2;

        while *p_s1 != 0 && *p_s1 == *p_s2 {
            p_s1 = p_s1.add(1);
            p_s2 = p_s2.add(1);
        }

        (*p_s1).cmp(&(*p_s2)) as i32
    }
}
