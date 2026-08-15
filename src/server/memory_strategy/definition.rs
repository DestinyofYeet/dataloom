use serde::{Serialize, de::DeserializeOwned};

use crate::server::memory_strategy::MemoryError;

pub trait MemoryStrategy: Send + Sync {
    fn store<T>(&self, item: T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.store_key(key, item)
    }

    fn store_key<T>(&self, key: &str, item: T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug;

    fn retrieve<T>(&self) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.retrieve_key(key)
    }

    fn retrieve_key<T>(&self, key: &str) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug;
}
