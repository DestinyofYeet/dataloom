use std::sync::Arc;

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{runnable_info::RunnableInfo, worker_logger::WorkerLogger},
};

impl<D> RunnableInfo<D>
where
    D: DatabaseStrategy,
{
    pub fn get_logger(&self) -> &WorkerLogger {
        &self.logger
    }

    pub fn get_database(&self) -> Arc<D> {
        self.database_handle.clone()
    }
}
