#![no_main]
#![no_std]
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;
mod sync;
// pub mod syscall;
pub mod trap;
mod syscall;
mod stack_trace;
mod loader;
mod task;
mod config;

use core::arch::global_asm;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[unsafe(no_mangle)]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    log_system_info();

    trap::init();
    loader::load_apps();
    task::run_first_task();
    panic!("Unreachable in rust_main!");
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
