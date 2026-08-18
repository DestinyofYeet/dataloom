use std::sync::Arc;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        runnable_info::RunnableInfo, taskhandler::task_actions::TaskActions,
        worker_logger::WorkerLogger,
    },
};

impl<'a, D, ME> RunnableInfo<'a, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(
        logger: WorkerLogger,
        database_handle: &'a D,
        memory_handle: &'a ME,
        task_actions: Arc<TaskActions<D, ME>>,
    ) -> Self {
        Self {
            logger,
            task_actions,
            database_handle,
            memory_handle,
        }
    }
}
