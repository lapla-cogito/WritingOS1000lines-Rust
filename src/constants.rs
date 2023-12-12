use core::ffi::c_void;

pub type SbiErr = isize;
pub const SBI_SUCCESS: isize = 0;
pub const SBI_ERR_FAILED: isize = -1;
pub const SBI_ERR_NOT_SUPPORTED: isize = -2;
pub const SBI_ERR_INVALID_PARAM: isize = -3;
pub const SBI_ERR_DENIED: isize = -4;
pub const SBI_ERR_INVALID_ADDRESS: isize = -5;
pub const SBI_ERR_ALREADY_AVAILABLE: isize = -6;
pub const SBI_ERR_ALREADY_STARTED: isize = -7;
pub const SBI_ERR_ALREADY_STOPPED: isize = -8;
pub const PAGE_SIZE: usize = 4096;

pub type Bool = i32;
pub type SizeT = u32;
pub type PaddrT = u32;
pub type VaddrT = u32;

pub const NULL: *const c_void = core::ptr::null();

pub const PROCS_MAX: usize = 8;
pub const PROC_UNUSED: i32 = 0;
pub const PROC_READY: i32 = 1;
