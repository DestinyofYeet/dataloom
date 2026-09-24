use std::sync::{Arc, Mutex};

use cron::Schedule;
use dataloom_db_core::traits::DatabaseStrategy;

use crate::{server::memory_strategy::MemoryStrategy, tasks::task::Task};

pub struct ReoccuringTask<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) description: String,
    pub(crate) run_at_startup: bool,
    pub(crate) schedule: Schedule,
    pub(crate) task: Arc<Mutex<Task<D, ME>>>,
}
