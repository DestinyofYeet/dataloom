use std::time::Duration;

use chrono::{DateTime, Utc};
use dataloom_db_core::traits::DatabaseStrategy;
use tracing::warn;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::default_tasks::internal::cron_interval::CronWorker,
};

impl<D, ME> CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) fn get_next_task(&self) -> Option<(Duration, Vec<u64>)> {
        let mut next_tasks: Option<(DateTime<Utc>, Vec<u64>)> = None;
        for (idx, task) in self
            .tasks
            .lock()
            .expect("to be able to get lock")
            .iter()
            .enumerate()
        {
            for next_time in task.schedule.upcoming(Utc).take(1) {
                let idx = idx as u64;

                match next_tasks {
                    None => next_tasks = Some((next_time, vec![idx])),

                    Some((curr_lowest, mut tasks)) => {
                        if next_time == curr_lowest {
                            tasks.push(idx);
                            next_tasks = Some((curr_lowest, tasks));
                        } else if next_time < curr_lowest {
                            next_tasks = Some((next_time, vec![idx]))
                        } else {
                            next_tasks = Some((curr_lowest, tasks))
                        }
                    }
                }
            }
        }

        if let Some((time, tasks)) = next_tasks {
            let now = Utc::now();

            let diff = time.signed_duration_since(now).num_milliseconds();

            if diff < 0 {
                warn!(
                    "next runnables with indexs {tasks:?} is scheduled to run in {diff}s. This should not be possible!"
                );
                return None;
            }

            Some((
                Duration::from_millis(diff.try_into().expect("to convert to u64")),
                tasks,
            ))
        } else {
            None
        }
    }
}
