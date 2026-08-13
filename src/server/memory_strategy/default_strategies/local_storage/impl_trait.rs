use serde::de::DeserializeOwned;

use crate::server::memory_strategy::{
    MemoryError, MemoryStrategy, default_strategies::local_storage::definition::LocalMemory,
};

impl MemoryStrategy for LocalMemory {
    fn store<T>(&self, key: &str, item: T) -> Result<(), MemoryError>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        let string =
            serde_json::to_string(&item).map_err(|e| MemoryError::Storage(e.to_string()))?;

        let mut map = self
            .storage
            .lock()
            .map_err(|e| MemoryError::Storage(format!("Failed to get lock: {e}")))?;

        map.insert(key.to_string(), string);

        Ok(())
    }

    fn retrieve<T>(
        &self,
        key: &str,
    ) -> Result<Option<T>, crate::server::memory_strategy::MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let map = self
            .storage
            .lock()
            .map_err(|e| MemoryError::Retrieve(format!("Failed to get lock {e}")))?;

        let result = match map.get(key) {
            Some(value) => {
                let item: T = serde_json::from_str(value)
                    .map_err(|e| MemoryError::Retrieve(e.to_string()))?;

                Some(item)
            }
            None => None,
        };

        Ok(result)
    }
}
