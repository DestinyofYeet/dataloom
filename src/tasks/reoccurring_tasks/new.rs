use roxygen::roxygen;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use cron::Schedule;
use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        reoccurring_tasks::{ReoccuringTask, ReoccuringTaskError},
        task::Task,
        taskrunnable::TaskRunnable,
    },
};

impl<D, ME> ReoccuringTask<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    #[roxygen]
    pub fn new<T>(
        description: impl Into<String>,

        /// A cron expression using the `cron` crate
        cron_expr: &str,

        /// The runnable that will run when the expression fires.
        /// This will run multiple times on the same object
        runnable: T,
    ) -> Result<Self, ReoccuringTaskError>
    where
        T: TaskRunnable<D, ME> + Send + Sync + 'static,
    {
        let schedule =
            Schedule::from_str(cron_expr).map_err(|e| ReoccuringTaskError::Cron(e.to_string()))?;

        let description = description.into();

        Ok(Self {
            run_at_startup: false,
            description,
            schedule,
            task: Arc::new(Mutex::new(Task::new(Box::new(runnable)))),
        })
    }
}
