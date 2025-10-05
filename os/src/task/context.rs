//! Implementation of [`TaskContext`]

/// Task Context
/// 实现了 Clone Trait 之后就可以调用 clone 函数完成拷贝；
///
/// 实现了 PartialEq Trait 之后就可以使用 == 运算符比较该类型的两个实例，从逻辑上说只有两个相等的应用执行状态才会被判为相等，而事实上也确实如此。
///
/// Copy 是一个标记 Trait，决定该类型在按值传参/赋值的时候采用移动语义还是复制语义。
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    /// return address ( e.g. __restore ) of __switch ASM function
    ra: usize,
    /// kernel stack pointer of app
    sp: usize,
    /// callee saved registers:  s 0..11
    s: [usize; 12],
}

impl TaskContext {
    /// init task context
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    /// set task context {__restore ASM funciton, kernel stack, s_0..12 }
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        unsafe extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}