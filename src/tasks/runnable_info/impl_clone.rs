use dataloom_db_core::traits::DatabaseStrategy;

use crate::{server::memory_strategy::MemoryStrategy, tasks::runnable_info::RunnableInfo};

impl<D, ME> Clone for RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn clone(&self) -> Self {
        Self {
            logger: self.logger.clone(),
            database_handle: self.database_handle.clone(),
            memory_handle: self.memory_handle.clone(),
            task_actions: self.task_actions.clone(),
        }
    }
}
