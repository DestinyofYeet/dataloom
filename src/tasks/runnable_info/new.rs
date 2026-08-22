use std::sync::Arc;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        runnable_info::RunnableInfo, taskhandler::task_actions::TaskActions,
        worker_logger::WorkerLogger,
    },
};

impl<D, ME> RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(
        logger: WorkerLogger,
        database_handle: Arc<D>,
        memory_handle: Arc<ME>,
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
