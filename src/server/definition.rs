use std::sync::Arc;

use crate::{
    server::{ServerError, database_strategy::DatabaseStrategy},
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct DataloomServer<D>
where
    D: DatabaseStrategy,
{
    task_handler: TaskHandler<D>,
    database_strategy: Arc<D>,
    has_shutdown: bool,
}

impl<D> DataloomServer<D>
where
    D: DatabaseStrategy + 'static,
{
    pub fn new(
        workers: u64,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: D,
    ) -> Result<Self, ServerError> {
        let db = Arc::new(database_strategy);
        Ok(Self {
            task_handler: TaskHandler::new(workers, Arc::new(logging_strategy), db.clone()),
            database_strategy: db.clone(),
            has_shutdown: false,
        })
    }

    pub fn get_database(&self) -> Arc<D> {
        self.database_strategy.clone()
    }

    pub fn get_task_handler(&self) -> &TaskHandler<D> {
        &self.task_handler
    }

    pub fn shutdown(&mut self) -> Result<(), ServerError> {
        if !self.has_shutdown {
            self.task_handler.shutdown()?;
        }
        Ok(())
    }
}
