use std::sync::Arc;

use crate::{
    server::{
        ServerError,
        database_strategy::DatabaseStrategy,
        memory_strategy::{self, MemoryStrategy},
    },
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct DataloomServer<D, M>
where
    D: DatabaseStrategy,
    M: MemoryStrategy,
{
    task_handler: TaskHandler<D, M>,
    database_strategy: Arc<D>,
    memory_strategy: Arc<M>,
    has_shutdown: bool,
}

impl<D, M> DataloomServer<D, M>
where
    D: DatabaseStrategy + 'static,
    M: MemoryStrategy + 'static,
{
    pub fn new(
        workers: u64,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: D,
        memory_strategy: M,
    ) -> Result<Self, ServerError> {
        let db = Arc::new(database_strategy);
        let mem = Arc::new(memory_strategy);
        Ok(Self {
            task_handler: TaskHandler::new(
                workers,
                Arc::new(logging_strategy),
                db.clone(),
                mem.clone(),
            ),
            database_strategy: db.clone(),
            has_shutdown: false,
            memory_strategy: mem.clone(),
        })
    }

    pub fn get_database(&self) -> Arc<D> {
        self.database_strategy.clone()
    }

    pub fn get_task_handler(&self) -> &TaskHandler<D, M> {
        &self.task_handler
    }

    pub fn shutdown(&mut self) -> Result<(), ServerError> {
        if !self.has_shutdown {
            self.task_handler.shutdown()?;
        }
        Ok(())
    }
}
