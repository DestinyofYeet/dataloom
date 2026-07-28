use std::{any::Any, sync::mpsc::SendError};

use thiserror::Error;

use crate::{server::database_strategy::DatabaseStrategy, tasks::taskhandler::TaskEvent};

#[derive(Debug, Error)]
pub enum TaskHandlerError {
    #[error("Failed to send message: {0}")]
    SendError(String),

    #[error("Failed to join on thread")]
    Join,
}

impl<D> From<SendError<TaskEvent<D>>> for TaskHandlerError
where
    D: DatabaseStrategy,
{
    fn from(value: SendError<TaskEvent<D>>) -> Self {
        Self::SendError(value.to_string())
    }
}
