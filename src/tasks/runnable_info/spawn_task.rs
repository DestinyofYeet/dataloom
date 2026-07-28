use std::sync::{Arc, Mutex};

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{
        runnable_info::RunnableInfo,
        task::{Runnable, Task},
        taskhandler::{TaskEvent, TaskHandlerError},
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D> RunnableInfo<D>
where
    D: DatabaseStrategy,
{
    pub fn spawn_task<T>(&self, runnable: T) -> Result<(), TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D> + Send + Sync + 'static,
    {
        let runnable: Runnable<D> = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.logger.logger.clone())));
        self.to_handler.send(TaskEvent::ProcessTask(task))?;

        Ok(())
    }
}
