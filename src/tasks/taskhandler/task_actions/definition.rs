use std::sync::mpsc::Sender;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{logstrategy::LogStrategyType, taskhandler::TaskEvent},
};

pub struct TaskActions<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) to_task_handler: Sender<TaskEvent<D, ME>>,
    pub(super) log_strategy: LogStrategyType,
}
