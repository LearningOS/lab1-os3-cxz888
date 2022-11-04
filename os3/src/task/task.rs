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
}

impl TaskControlBlock {
    pub const fn uninit() -> Self {
        Self {
            task_ctx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
        }
    }
}
