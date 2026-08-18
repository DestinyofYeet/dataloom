use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        taskhandler::{
            TaskHandler, TaskHandlerError, task_actions::spawn_options::TaskSpawnOptions,
        },
        taskref::TaskRef,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn spawn_task<T>(&self, runnable: T) -> Result<TaskRef<T, D, ME>, TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable<D, ME> + Send + Sync + 'static,
    {
        self.task_actions
            .spawn_task(runnable, TaskSpawnOptions::new())
    }
}
