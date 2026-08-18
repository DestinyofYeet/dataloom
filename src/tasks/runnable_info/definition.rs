use std::sync::Arc;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{taskhandler::task_actions::TaskActions, worker_logger::WorkerLogger},
};

pub struct RunnableInfo<'a, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) logger: WorkerLogger,
    pub(super) database_handle: &'a D,
    pub(super) memory_handle: &'a ME,
    pub(super) task_actions: Arc<TaskActions<D, ME>>,
}
