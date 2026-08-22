use std::sync::Arc;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{taskhandler::task_actions::TaskActions, worker_logger::WorkerLogger},
};

pub struct RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) logger: WorkerLogger,
    pub(super) database_handle: Arc<D>,
    pub(super) memory_handle: Arc<ME>,
    pub(super) task_actions: Arc<TaskActions<D, ME>>,
}
