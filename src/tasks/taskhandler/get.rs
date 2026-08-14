use std::sync::Arc;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::taskhandler::{TaskHandler, task_actions::TaskActions},
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn get_task_actions(&self) -> Arc<TaskActions<D, ME>> {
        self.task_actions.clone()
    }
}
