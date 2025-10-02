use std::arch::asm;

fn main() {
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
    while !fp.is_null() {
        // fp[-1] = return address
        // fp[-2] = previous stack pointer (which is also the previous fp in standard ABI)
        let ret_addr = unsafe { *fp.sub(1) };
        let prev_fp = unsafe { *fp.sub(2) as *const usize };

        println!("Return address: 0x{:016x}", ret_addr);
        println!("Old frame pointer: 0x{:016x}", prev_fp as usize);
        println!();

        fp = prev_fp;
    }

    println!("=== End ===\n");
}