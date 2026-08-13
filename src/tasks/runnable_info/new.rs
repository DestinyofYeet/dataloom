use std::sync::{Arc, mpsc::Sender};

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{runnable_info::RunnableInfo, taskhandler::TaskEvent, worker_logger::WorkerLogger},
};

impl<D, ME> RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(
        logger: WorkerLogger,
        to_handler: Sender<TaskEvent<D, ME>>,
        database_handle: Arc<D>,
        memory_handle: Arc<ME>,
    ) -> Self {
        Self {
            logger,
            to_handler,
            database_handle,
            memory_handle,
        }
    }
}
