use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReoccuringTaskError {
    #[error("Invalid cron expression: {0}")]
    Cron(String),
}
