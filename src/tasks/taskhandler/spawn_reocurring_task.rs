use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{reoccurring_tasks::ReoccuringTask, taskhandler::TaskHandler},
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn spawn_reocurring_task(&self, task: ReoccuringTask<D, ME>) {
        self.reoccuring_tasks
            .lock()
            .expect("to get lock")
            .push(task);
    }
}
