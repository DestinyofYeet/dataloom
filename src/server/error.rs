use thiserror::Error;

use crate::tasks::taskhandler::TaskHandlerError;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("{0}")]
    TaskHandler(#[from] TaskHandlerError),

    #[error("Failed to query available parallelism: {0}")]
    Parallelism(String),
}
