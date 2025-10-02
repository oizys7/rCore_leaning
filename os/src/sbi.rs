pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}

// pub fn console_getchar() -> Option<usize> {
//     #[allow(deprecated)]
//     let ret = sbi_rt::legacy::console_getchar();
//     // -1 的二进制表示 = 全 1 = usize::MAX
//     if ret == usize::MAX {
//         None // 无输入
//     } else {
//         Some(ret as usize) // 有输入，转为 u8（ASCII）
//     }
// }

pub fn shutdown(failure: bool) -> ! {
    use sbi_rt::{NoReason, Shutdown, SystemFailure, system_reset};
    if !failure {
        system_reset(Shutdown, NoReason);
    } else {
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
