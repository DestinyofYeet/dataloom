use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        taskhandler::{TaskHandler, TaskHandlerError},
        taskref::TaskRef,
    },
};

impl<'a, D, ME> TaskHandler<'a, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy + 'static,
{
    pub fn wait_until_done<T>(&self, task: &TaskRef<T, D, ME>) -> Result<(), TaskHandlerError> {
        self.task_actions.wait_until_done(task)
    }
}
