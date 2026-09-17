use std::time::Duration;

use chrono::{DateTime, Utc};
use dataloom_db_core::traits::DatabaseStrategy;
use itertools::Itertools;
use tracing::warn;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::internal::cron_interval::CronWorker, reoccurring_tasks::ReoccuringTask,
    },
};

impl<D, ME> CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) fn get_next_task(tasks: &[ReoccuringTask<D, ME>]) -> Option<(Duration, Vec<u64>)> {
        let mut next_tasks: Option<(DateTime<Utc>, Vec<(u64, &ReoccuringTask<D, ME>)>)> = None;
        for (idx, task) in tasks.iter().enumerate() {
            let idx = idx as u64;
            for next_time in task.schedule.upcoming(Utc).take(1) {
                match next_tasks {
                    None => next_tasks = Some((next_time, vec![(idx, task)])),

                    Some((curr_lowest, mut tasks)) => {
                        if next_time == curr_lowest {
                            tasks.push((idx, task));
                            next_tasks = Some((curr_lowest, tasks));
                        } else if next_time < curr_lowest {
                            next_tasks = Some((next_time, vec![(idx, task)]))
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
                    "next runnables with names {} is scheduled to run in {diff}s. This should not be possible!",
                    tasks.iter().map(|(_, e)| e.description.clone()).join(", ")
                );
                return None;
            }

            Some((
                Duration::from_millis(diff.try_into().expect("to convert to u64")),
                tasks.into_iter().map(|(idx, _)| idx).collect_vec(),
            ))
        } else {
            None
        }
    }
}
