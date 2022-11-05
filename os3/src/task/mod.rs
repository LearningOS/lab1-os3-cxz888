use core::mem;

use lazy_static::lazy_static;

pub use self::task::TaskStatus;
use self::{context::TaskContext, switch::__switch, task::TaskControlBlock};
use crate::{
    config::{MAX_APP_NUM, MAX_SYSCALL_NUM},
    loader,
    sync::UPSafeCell,
    timer,
};

pub mod context;
pub mod switch;
mod task;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = {
        let num_app = loader::get_num_app();
        const UNINIT_TCB: TaskControlBlock = TaskControlBlock::uninit();
        let task_manager = TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks: [UNINIT_TCB; MAX_APP_NUM],
                    current_task: 0,
                })
            },
        };
        for (i, t) in task_manager
            .inner
            .exclusive_access()
            .tasks
            .iter_mut()
            .enumerate()
            .take(num_app)
        {
            t.task_ctx = TaskContext::goto_restore(loader::init_app_ctx(i));
            t.task_status = TaskStatus::Ready;
            t.task_start_time = timer::get_time_ms();
        }
        task_manager
    };
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    tasks: [TaskControlBlock; MAX_APP_NUM],
    current_task: usize,
}

impl TaskManager {
    fn mark_current(&self, status: TaskStatus) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = status;
    }
    fn next_ready_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        let is_ready = |&id: &usize| inner.tasks[id].task_status == TaskStatus::Ready;
        (current + 1..self.num_app)
            .find(is_ready)
            .or((0..=current).find(is_ready))
    }
    fn run_next_task(&self) {
        if let Some(next) = self.next_ready_task() {
            let current_task_ctx_ptr;
            let next_task_ctx_ptr;
            {
                let mut inner = self.inner.exclusive_access();
                let current = inner.current_task;
                inner.tasks[next].task_status = TaskStatus::Running;
                inner.current_task = next;
                current_task_ctx_ptr = &mut inner.tasks[current].task_ctx as *mut TaskContext;
                next_task_ctx_ptr = &inner.tasks[next].task_ctx as *const TaskContext;
            }
            unsafe {
                __switch(current_task_ctx_ptr, next_task_ctx_ptr);
            }
        } else {
            panic!("All application completed!");
        }
    }

    fn run_first_task(&self) -> ! {
        let next_task_ctx_ptr;
        {
            let mut inner = self.inner.exclusive_access();
            let task0 = &mut inner.tasks[0];
            task0.task_status = TaskStatus::Running;
            next_task_ctx_ptr = &task0.task_ctx as *const TaskContext;
        }
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_ctx_ptr);
        }
        unreachable!();
    }
    pub fn get_start_time(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.tasks[inner.current_task].task_start_time
    }
}

pub fn suspend_current_and_run_next() {
    TASK_MANAGER.mark_current(TaskStatus::Ready);
    TASK_MANAGER.run_next_task();
}

pub fn exit_current_and_run_next() {
    TASK_MANAGER.mark_current(TaskStatus::Exited);
    TASK_MANAGER.run_next_task();
}

pub fn run_first_task() {
    debug!(
        "Address of task manager: {:p} {:p};size: {};size of TCB: {}",
        &TASK_MANAGER.inner as *const _,
        &TASK_MANAGER.num_app as *const _,
        mem::size_of_val(&TASK_MANAGER.inner),
        mem::size_of::<TaskControlBlock>()
    );
    TASK_MANAGER.run_first_task()
}

pub fn get_start_time() -> usize {
    TASK_MANAGER.get_start_time()
}

/// 假定 syscall_id < MAX_SYSCALL_NUM
pub fn increment_syscall_times(syscall_id: usize) {
    let mut inner = TASK_MANAGER.inner.exclusive_access();
    let curr_task = inner.current_task;
    inner.tasks[curr_task].task_syscall_times[syscall_id] += 1;
}

pub fn set_syscall_times(dst: &mut [u32; MAX_SYSCALL_NUM]) {
    let inner = TASK_MANAGER.inner.exclusive_access();
    let curr_task = inner.current_task;
    dst.copy_from_slice(&inner.tasks[curr_task].task_syscall_times)
}
