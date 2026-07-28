use std::sync::{Arc, Mutex};

use crate::tasks::{
    runnable_info::RunnableInfo,
    task::{Runnable, Task},
    taskhandler::{TaskEvent, TaskHandlerError},
    taskrunnable::{TaskResultable, TaskRunnable},
};

impl RunnableInfo {
    pub fn spawn_task<T>(&self, runnable: T) -> Result<(), TaskHandlerError>
    where
        T: TaskResultable + TaskRunnable + Send + Sync + 'static,
    {
        let runnable: Runnable = Box::new(runnable);
        let task = Arc::new(Mutex::new(Task::new(runnable, self.logger.logger.clone())));
        self.to_handler.send(TaskEvent::ProcessTask(task))?;

        Ok(())
    }
}
