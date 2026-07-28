use std::any::Any;

use crate::tasks::{runnable_info::RunnableInfo, task::TaskResult, worker_logger::WorkerLogger};

pub trait TaskRunnable {
    fn run(&mut self, info: RunnableInfo) -> Box<dyn Any + Send + Sync>;
}

pub trait TaskResultable {
    type Result;

    fn downcast(result: TaskResult) -> Self::Result;
}
