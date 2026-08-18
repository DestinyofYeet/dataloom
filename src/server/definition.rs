use std::sync::Arc;

use crate::{
    server::{ServerError, database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{logstrategy::LogStrategy, taskhandler::TaskHandler},
};

pub struct DataloomServer<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    task_handler: TaskHandler<D, ME>,
    database_strategy: &'static D,
    memory_strategy: &'static ME,
    has_shutdown: bool,
}

impl<D, ME> DataloomServer<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub fn new(
        workers: u64,
        logging_strategy: impl LogStrategy + Send + Sync + 'static,
        database_strategy: D,
        memory_strategy: ME,
    ) -> Result<Self, ServerError> {
        let db = Box::leak(Box::new(database_strategy));
        let mem = Box::leak(Box::new(memory_strategy));
        Ok(Self {
            task_handler: TaskHandler::new(workers, Arc::new(logging_strategy), db, mem),
            database_strategy: db,
            has_shutdown: false,
            memory_strategy: mem,
        })
    }

    pub fn get_database<'a>(&self) -> &'a D
    where
        Self: 'a,
    {
        self.database_strategy
    }

    pub fn get_memory<'a>(&self) -> &'a ME
    where
        Self: 'a,
    {
        self.memory_strategy
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
