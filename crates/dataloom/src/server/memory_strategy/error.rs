use thiserror::Error;

#[derive(Debug, Error)]
pub enum MemoryError {
    #[error("Failed to store item: {0}")]
    Storage(String),

    #[error("Failed to retrieve item: {0}")]
    Retrieve(String),
}
