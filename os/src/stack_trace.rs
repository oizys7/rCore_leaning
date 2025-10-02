use core::{arch::asm, ptr};

pub unsafe fn print_stack_trace() -> () {
    println!("=== Stack trace from fp chain ===");

    let mut fp: *const usize;
    unsafe {
        asm!(
        "mv {}, fp",
        out(reg) fp,
        options(nostack, preserves_flags),
        );
    }

    // Traverse the frame pointer chain
    while fp != ptr::null() {
        // fp[-1] = return address
        // fp[-2] = previous stack pointer (which is also the previous fp in standard ABI)
        let ret_addr =  *fp.sub(1);
        let prev_fp =  *fp.sub(2);

        println!("0x{:016x}, fp = 0x{:016x}", ret_addr, prev_fp);
        fp = prev_fp as *const usize;
    }
    println!("=== End of Stack trace ===\n");
}