use std::sync::Arc;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::{ServerError, memory_strategy::MemoryStrategy},
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct DataloomServer<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    task_handler: TaskHandler<D, ME>,
    database_strategy: Arc<D>,
    memory_strategy: Arc<ME>,
    has_shutdown: bool,
}

impl<D, ME> DataloomServer<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub fn new(
        workers: impl Into<Option<u64>>,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: D,
        memory_strategy: ME,
    ) -> Result<Self, ServerError> {
        let db = Arc::new(database_strategy);
        let mem = Arc::new(memory_strategy);

        let worker_count = workers.into().unwrap_or(
            std::thread::available_parallelism()
                .map_err(|e| ServerError::Parallelism(e.to_string()))?
                .get() as u64,
        );

        Ok(Self {
            task_handler: TaskHandler::new(
                worker_count,
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

    pub fn get_memory(&self) -> Arc<ME> {
        self.memory_strategy.clone()
    }

    pub fn get_task_handler(&self) -> &TaskHandler<D, ME> {
        &self.task_handler
    }

    pub fn shutdown(&mut self) -> Result<(), ServerError> {
        if !self.has_shutdown {
            self.task_handler.shutdown()?;
        }
        Ok(())
    }
}
