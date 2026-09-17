use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::internal::cron_interval::CronWorker,
        reoccurring_tasks::ReoccuringTask,
        taskhandler::{TaskHandler, TaskHandlerError},
    },
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub fn spawn_reocurring_task(
        &self,
        task: ReoccuringTask<D, ME>,
    ) -> Result<(), TaskHandlerError> {
        let mut tasks = self.reoccuring_tasks.lock().expect("to get lock");

        if !tasks.is_empty() {
            tasks.push(task);
            return Ok(());
        }

        tasks.push(task);
        drop(tasks);

        let _ = self.spawn_task_long_running(CronWorker::new(
            self.reoccuring_tasks.clone(),
            self.to_handler.clone(),
        ))?;

        Ok(())
    }
}
