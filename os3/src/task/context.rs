#[repr(C)]
#[derive(Clone)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    // callee-saved 寄存器
    s: [usize; 12],
}

impl TaskContext {
    pub const fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
    pub fn goto_restore(kernel_stack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kernel_stack_ptr,
            s: [0; 12],
        }
    }
}
