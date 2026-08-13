use std::any::Any;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{runnable_info::RunnableInfo, task::TaskResult, worker_logger::WorkerLogger},
};

pub trait TaskRunnable<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn run(&mut self, info: RunnableInfo<D, ME>) -> Box<dyn Any + Send + Sync>;
}

pub trait TaskResultable {
    type Result;

    fn downcast(result: TaskResult) -> Self::Result;
}
