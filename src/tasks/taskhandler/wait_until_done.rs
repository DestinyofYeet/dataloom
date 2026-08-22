use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        taskhandler::{TaskHandler, TaskHandlerError},
        taskref::TaskRef,
    },
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy + 'static,
{
    pub fn wait_until_done<T>(&self, task: &TaskRef<T, D, ME>) -> Result<(), TaskHandlerError> {
        self.task_actions.wait_until_done(task)
    }
}
