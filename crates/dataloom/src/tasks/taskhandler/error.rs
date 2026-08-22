use std::sync::mpsc::SendError;

use thiserror::Error;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::taskhandler::TaskEvent,
};

#[derive(Debug, Error)]
pub enum TaskHandlerError {
    #[error("Failed to send message: {0}")]
    SendError(String),

    #[error("Failed to join on thread")]
    Join,
}

impl<D, ME> From<SendError<TaskEvent<D, ME>>> for TaskHandlerError
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn from(value: SendError<TaskEvent<D, ME>>) -> Self {
        Self::SendError(value.to_string())
    }
}
