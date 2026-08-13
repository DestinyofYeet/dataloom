use std::sync::{Arc, Mutex};

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        task::{Runnable, Task},
        taskhandler::{TaskEvent, TaskHandler, TaskHandlerError},
        taskref::TaskRef,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn spawn_task_long_running<T>(
        &self,
        runnable: T,
    ) -> Result<TaskRef<T, D, ME>, TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D, ME> + Send + Sync + 'static,
    {
        let runnable: Runnable<D, ME> = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.log_strategy.clone())));

        let task_ref = TaskRef::new(task.clone());
        self.to_handler.send(TaskEvent::ProcessLongTask(task))?;
        Ok(task_ref)
    }
}
