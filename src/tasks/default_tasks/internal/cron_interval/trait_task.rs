use std::time::Duration;

use dataloom_db_core::traits::DatabaseStrategy;
use tracing::{debug, error, info};

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::internal::cron_interval::CronWorker,
        runnable_info::RunnableInfo,
        taskhandler::TaskEvent,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

impl<D, ME> TaskRunnable<D, ME> for CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn run(&mut self, run_info: RunnableInfo<D, ME>) -> Box<dyn std::any::Any + Send + Sync> {
        loop {
            let (next_duration, next_tasks_idx) = match self.get_next_task() {
                None => {
                    debug!(
                        "No cron tasks found. Sleeping. list: {}",
                        self.tasks.lock().expect("to get lock").len()
                    );
                    std::thread::sleep(Duration::from_secs(1));
                    continue;
                }
                Some(value) => value,
            };

            if next_duration.as_millis() == 0 {
                continue;
            }

            info!(
                "Sleeping for {:.2}s for next task run.",
                next_duration.as_secs_f32()
            );

            std::thread::sleep(next_duration);

            let tasks = self.tasks.lock().expect("to get lock");

            for task_idx in next_tasks_idx {
                let task = tasks.get(task_idx as usize).expect("to have a task");

                match self.sender.send(TaskEvent::ProcessTask(task.task.clone())) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed to send scheduled task to main pool: {e}");
                    }
                }
            }
        }
    }
}

impl<D, ME> TaskResultable for CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    type Result = ();

    fn downcast(_: crate::tasks::task::TaskResult) -> Self::Result {}
}
