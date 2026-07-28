use std::any::Any;

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{runnable_info::RunnableInfo, task::TaskResult, worker_logger::WorkerLogger},
};

pub trait TaskRunnable<D>
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Send + Sync>;
}

pub trait TaskResultable {
    type Result;

    fn downcast(result: TaskResult) -> Self::Result;
}
