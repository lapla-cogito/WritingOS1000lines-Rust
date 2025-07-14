pub static mut NEXT_PADDR: u32 = 0;

pub unsafe fn alloc_pages(n: u32) -> crate::constants::PaddrT {
    let paddr = NEXT_PADDR;
    NEXT_PADDR += n * crate::constants::PAGE_SIZE as u32;

    if NEXT_PADDR > core::ptr::addr_of!(crate::__free_ram_end) as crate::constants::PaddrT {
        panic!("out of memory");
    }

    core::ptr::write_bytes(
        paddr as *mut u8,
        0,
        (n * crate::constants::PAGE_SIZE as u32) as usize,
    );
    paddr
}

pub unsafe fn map_page(
    table1: crate::constants::PaddrT,
    vaddr: crate::constants::VaddrT,
    paddr: crate::constants::PaddrT,
    flags: u32,
) {
    if !vaddr.is_multiple_of(crate::constants::PAGE_SIZE as u32) {
        panic!("unaligned vaddr {:x}", vaddr);
    }

    if !vaddr.is_multiple_of(crate::constants::PAGE_SIZE as u32) {
        panic!("unaligned paddr {:x}", paddr);
    }

    let table1 = table1 as *mut u32;
    let vpn1 = ((vaddr >> 22) & 0x3ff) as isize;
    if *table1.offset(vpn1) & crate::constants::PAGE_V == 0 {
        let pt_paddr = alloc_pages(1);
        table1.offset(vpn1).write(
            (((pt_paddr / crate::constants::PAGE_SIZE as u32) as u32) << 10)
                | crate::constants::PAGE_V,
        );
    }

    let table0 = ((*table1.offset(vpn1) >> 10) * crate::constants::PAGE_SIZE as u32) as *mut u32;
    let vpn0 = ((vaddr >> 12) & 0x3ff) as isize;
    table0.offset(vpn0).write(
        ((paddr / crate::constants::PAGE_SIZE as u32) << 10) | flags | crate::constants::PAGE_V,
    );
}
