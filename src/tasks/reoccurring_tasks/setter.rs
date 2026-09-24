use dataloom_db_core::traits::DatabaseStrategy;

use crate::{server::memory_strategy::MemoryStrategy, tasks::reoccurring_tasks::ReoccuringTask};

impl<D, ME> ReoccuringTask<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn run_at_startup(mut self, value: bool) -> Self {
        self.run_at_startup = value;

        self
    }
}
