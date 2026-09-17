use std::time::Duration;

use chrono::Utc;
use dataloom_db_core::traits::DatabaseStrategy;
use itertools::Itertools;
use tracing::{debug, error, info, trace};

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
    fn run(&mut self, _: RunnableInfo<D, ME>) -> Box<dyn std::any::Any + Send + Sync> {
        loop {
            let tasks = self.tasks.lock().expect("to get lock");
            let (next_duration, next_tasks) = match Self::get_next_task(&tasks) {
                None => {
                    unreachable!(
                        "The CronWorker should only be instanciated if it has reocurring events."
                    )
                }
                Some(value) => value,
            };

            // In case a we already scheduled the task to run,
            // but are still in the trigger window, the duration will be 0
            if next_duration.as_millis() == 0 {
                trace!("next_duration is 0");
                let now = Utc::now();
                let nanos_to_next_second = 999_999_999 - now.timestamp_subsec_nanos();
                trace!("sleeping {nanos_to_next_second}ns");
                std::thread::sleep(Duration::from_nanos(nanos_to_next_second as u64));
                continue;
            }

            info!(
                "Sleeping for {:.2}s for following tasks: {}",
                next_duration.as_secs_f32(),
                format!(
                    "[{}]",
                    next_tasks
                        .iter()
                        .map(|e| tasks.get(*e as usize).unwrap().description.clone())
                        .join(", ")
                )
            );

            drop(tasks);

            std::thread::sleep(next_duration);

            let tasks = self.tasks.lock().expect("to get lock");
            for task_idx in next_tasks {
                let task = tasks.get(task_idx as usize).expect("to have task");

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
