#![no_std]
#![no_main]

#[no_mangle]
fn main() {
    libos::init();

    libos::println!("\n[ArceOS Tutorial]: A0");
    verify();
}

fn verify() {
    /*
     * The first two instrcutions at kernel entry:
     * ffffffc080200000:  842a  mv s0, a0
     * ffffffc080200002:  84ae  mv s1, a1
     */
    const THE_FIRST_TWO_INSTRUCTIONS: u32 = 0x84ae_842a;

    // start address of kernel
    let base = libos::KERNEL_BASE + 0x20_0000;

    unsafe {
        let p: *const u32 = base as *const u32;
        if *p != THE_FIRST_TWO_INSTRUCTIONS {
            libos::println!("Result: bad instructions {:x}", *p);
            return;
        }
    }

    libos::println!("Result: Okay!");
}
