mod context;
mod pid;
mod process;
mod signal;
mod task;
mod manager;
mod processor;

pub use context::TaskContext;
pub use pid::{PidHandle, pid_alloc};
pub use manager::{add_task, pid2process, remove_from_pid2process, wakeup_task};
pub use processor::{
    current_kstack_top, current_process, current_task, current_trap_cx, current_trap_cx_user_va,
    current_user_token, run_tasks, schedule, take_current_task,
};
pub use signal::SignalFlags;
pub use task::{TaskControlBlock, TaskStatus};

pub fn suspend_current_and_run_next() {
    todo!()
}

pub fn block_current_and_run_next() {
    todo!()
}

pub fn block_current_task() -> *mut TaskContext {
    todo!()
}

