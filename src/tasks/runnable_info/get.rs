use std::sync::Arc;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{runnable_info::RunnableInfo, worker_logger::WorkerLogger},
};

impl<D, ME> RunnableInfo<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn get_logger(&self) -> &WorkerLogger {
        &self.logger
    }

    pub fn get_database(&self) -> Arc<D> {
        self.database_handle.clone()
    }

    pub fn get_memory(&self) -> Arc<ME> {
        self.memory_handle.clone()
    }
}
