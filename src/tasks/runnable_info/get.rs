use std::sync::Arc;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        runnable_info::RunnableInfo, taskhandler::task_actions::TaskActions,
        worker_logger::WorkerLogger,
    },
};

impl<'a, D, ME> RunnableInfo<'a, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn get_logger(&self) -> &WorkerLogger {
        &self.logger
    }

    pub fn get_database(&self) -> &D {
        self.database_handle
    }

    pub fn get_memory(&self) -> &ME {
        self.memory_handle
    }

    pub fn get_task_actions(&self) -> &Arc<TaskActions<D, ME>> {
        &self.task_actions
    }
}
