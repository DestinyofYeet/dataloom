use std::sync::{Arc, Mutex};

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        runnable_info::RunnableInfo,
        task::{Runnable, Task},
        taskhandler::{TaskEvent, TaskHandlerError},
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D, ME> RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn spawn_task<T>(&self, runnable: T) -> Result<(), TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D, ME> + Send + Sync + 'static,
    {
        let runnable: Runnable<D, ME> = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.logger.logger.clone())));
        self.to_handler.send(TaskEvent::ProcessTask(task))?;

        Ok(())
    }
}
