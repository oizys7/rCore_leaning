#![no_main]
#![no_std]
mod console;
mod lang_items;
mod sbi;
mod logging;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    log_system_info();

    println!("Hello, world!");
    // panic!("Shutdown machine!");
    sbi::shutdown(true);
}

fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

fn log_system_info () {
    unsafe extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
    }
    log::info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    log::info!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    log::info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    log::info!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
}
