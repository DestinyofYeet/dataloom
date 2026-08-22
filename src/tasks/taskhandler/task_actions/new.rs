use std::sync::mpsc::Sender;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        logstrategy::LogStrategyType,
        taskhandler::{TaskEvent, task_actions::TaskActions},
    },
};

impl<D, ME> TaskActions<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(
        to_task_handler: Sender<TaskEvent<D, ME>>,
        log_strategy: LogStrategyType,
    ) -> Self {
        Self {
            to_task_handler,
            log_strategy,
        }
    }
}
