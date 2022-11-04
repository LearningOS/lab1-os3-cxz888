use crate::task;

pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    task::exit_current_and_run_next();
    unreachable!();
}

/// APP 将 CPU 控制权交给 OS，由 OS 决定下一步。
///
/// 总是返回 0.
///
/// syscall ID: 124
pub fn sys_yield() -> isize {
    task::suspend_current_and_run_next();
    0
}
