use crate::{
    config::MAX_SYSCALL_NUM,
    task::{self, TaskStatus},
    timer,
};

use super::SYSCALL_TASK_INFO;

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

#[repr(C)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// `_tz` 在我们的实现中忽略
///
/// syscall ID: 169
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    unsafe {
        (*ts).usec = timer::get_time_us();
    }
    0
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// 查询任务信息。syscall_id = 410
///
/// 成功返回 0，错误返回 -1
///
/// NOTE: 但目前似乎没有错误的情况？
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    debug!("syscall task info");
    unsafe {
        (*ti).status = TaskStatus::Running;
        task::set_syscall_times(&mut (*ti).syscall_times);
        (*ti).time = timer::get_time_ms() - task::get_start_time();
    }
    0
}
