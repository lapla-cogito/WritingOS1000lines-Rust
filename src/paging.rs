use crate::{__free_ram_end, constants::*};
use core::ptr;

pub static mut NEXT_PADDR: u32 = 0;

pub unsafe fn alloc_pages(n: u32) -> PaddrT {
    let paddr = NEXT_PADDR;
    NEXT_PADDR += n * PAGE_SIZE as u32;

    if NEXT_PADDR > ptr::addr_of!(__free_ram_end) as PaddrT {
        panic!("out of memory");
    }

    ptr::write_bytes(paddr as *mut u8, 0, (n * PAGE_SIZE as u32) as usize);
    paddr
}

pub unsafe fn map_page(table1: PaddrT, vaddr: VaddrT, paddr: PaddrT, flags: u32) {
    if vaddr % PAGE_SIZE as u32 != 0 {
        panic!("unaligned vaddr {:x}", vaddr);
    }

    if vaddr % PAGE_SIZE as u32 != 0 {
        panic!("unaligned paddr {:x}", paddr);
    }

    let table1 = table1 as *mut u32;
    let vpn1 = ((vaddr >> 22) & 0x3ff) as isize;
    if *table1.offset(vpn1) & PAGE_V as u32 == 0 {
        let pt_paddr = alloc_pages(1);
        table1
            .offset(vpn1)
            .write((((pt_paddr / PAGE_SIZE as u32) as u32) << 10) | PAGE_V);
    }

    let table0 = ((*table1.offset(vpn1) >> 10) * PAGE_SIZE as u32) as *mut u32;
    let vpn0 = ((vaddr >> 12) & 0x3ff) as isize;
    table0
        .offset(vpn0)
        .write(((paddr / PAGE_SIZE as u32) << 10) as u32 | flags | PAGE_V as u32);
}
