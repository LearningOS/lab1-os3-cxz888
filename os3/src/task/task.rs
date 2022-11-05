use crate::config::MAX_SYSCALL_NUM;

use super::context::TaskContext;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Clone)]
pub struct TaskControlBlock {
    pub task_ctx: TaskContext,
    pub task_status: TaskStatus,
    pub task_start_time: usize,
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskControlBlock {
    pub const fn uninit() -> Self {
        Self {
            task_ctx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            task_start_time: 0,
            task_syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }
}
