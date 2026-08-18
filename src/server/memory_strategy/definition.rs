use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::server::memory_strategy::MemoryError;

pub trait MemoryStrategy: Send + Sync {
    fn store_key<T>(&'static self, key: &str, item: &T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug;

    fn retrieve_key<T>(&'static self, key: &str) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug;

    fn modify_key_mut<'a, T, F, RES>(
        &'static self,
        key: &str,
        func: F,
    ) -> Result<Option<RES>, MemoryError>
    where
        T: Deserialize<'a> + std::fmt::Debug + Serialize,
        F: FnOnce(&mut T) -> RES;

    fn store<T>(&'static self, item: &T) -> Result<(), MemoryError>
    where
        T: Serialize + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.store_key(key, &item)
    }

    fn retrieve<T>(&'static self) -> Result<Option<T>, MemoryError>
    where
        T: DeserializeOwned + std::fmt::Debug,
    {
        let key = std::any::type_name::<T>();

        self.retrieve_key(key)
    }
}
