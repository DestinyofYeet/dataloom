use std::sync::{Arc, mpsc::Sender};

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{taskhandler::TaskEvent, worker_logger::WorkerLogger},
};

pub struct RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) logger: WorkerLogger,
    pub(super) to_handler: Sender<TaskEvent<D, ME>>,
    pub(super) database_handle: Arc<D>,
    pub(super) memory_handle: Arc<ME>,
}
