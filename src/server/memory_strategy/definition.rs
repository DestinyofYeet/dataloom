use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::server::memory_strategy::MemoryError;

pub trait MemoryStrategy: Send + Sync {
    fn store_key<T>(&self, key: &str, item: &T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug;

    fn get_key<T>(&self, key: &str) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug;

    fn update_key<T, F, RES>(&self, key: &str, func: F) -> Result<RES, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug + Serialize,
        F: FnOnce(Option<&mut T>) -> RES;

    fn store<T>(&self, item: &T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.store_key(key, &item)
    }

    fn get<T>(&self) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.get_key(key)
    }

    fn update<T, F, RES>(&self, func: F) -> Result<RES, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug + Serialize,
        F: FnOnce(Option<&mut T>) -> RES,
    {
        let key = std::any::type_name::<T>();
        self.update_key(key, func)
    }
}
