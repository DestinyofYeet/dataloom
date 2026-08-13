use serde::{Serialize, de::DeserializeOwned};

use crate::server::memory_strategy::MemoryError;

pub trait MemoryStrategy: Send + Sync {
    fn store<T>(&self, key: &str, item: T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug;

    fn retrieve<T>(&self, key: &str) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug;
}
