use std::sync::{Arc, Mutex};

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        task::{Runnable, Task},
        taskhandler::{
            TaskEvent, TaskHandlerError,
            task_actions::{TaskActions, spawn_options::TaskSpawnOptions},
        },
        taskref::TaskRef,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D, ME> TaskActions<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy + 'static,
{
    pub fn spawn_task<T>(
        &self,
        runnable: T,
        options: TaskSpawnOptions,
    ) -> Result<TaskRef<T, D, ME>, TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D, ME> + Send + Sync + 'static,
    {
        let runnable: Runnable<D, ME> = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.log_strategy.clone())));

        let task_ref = TaskRef::new(task.clone());
        if options.is_long_running() {
            self.to_task_handler
                .send(TaskEvent::ProcessLongTask(task))?;
        } else {
            self.to_task_handler.send(TaskEvent::ProcessTask(task))?;
        }
        Ok(task_ref)
    }
}
