use std::sync::{Arc, Mutex};

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{
        task::{Runnable, Task},
        taskhandler::{TaskEvent, TaskHandler, TaskHandlerError},
        taskref::TaskRef,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D> TaskHandler<D>
where
    D: DatabaseStrategy,
{
    pub fn spawn_task<T>(&self, runnable: T) -> Result<TaskRef<T, D>, TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D> + Send + Sync + 'static,
    {
        let runnable: Runnable<D> = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.log_strategy.clone())));

        let task_ref = TaskRef::new(task.clone());
        self.to_handler.send(TaskEvent::ProcessTask(task))?;
        Ok(task_ref)
    }
}
