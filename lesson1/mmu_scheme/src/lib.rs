#![no_std]

pub const KERNEL_BASE: usize = 0xffff_ffff_c000_0000;

pub unsafe fn pre_mmu() {
    todo!("dummy");
}

pub unsafe fn enable_mmu() {
    todo!("dummy");
}

pub unsafe fn post_mmu() {
    todo!("dummy");
}
