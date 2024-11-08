//! Types related to task management

use crate::{config::MAX_SYSCALL_NUM, syscall::TaskInfo, timer::get_time_ms};

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// start time
    pub task_start_time: usize,
    /// syscall record
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskControlBlock{
    /// update syscall
    pub fn update_syscall(&mut self, syscall_id: usize) {
        self.syscall_times[syscall_id] += 1;
    }
    /// get task info
    pub fn task_info(&self, _ti: *mut TaskInfo) -> isize {
        unsafe {
            (*_ti).status = TaskStatus::Running;
            (*_ti).time = get_time_ms() - self.task_start_time;
            (*_ti).syscall_times = self.syscall_times;
        }
        0
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
