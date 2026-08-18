use serde::{Serialize, de::DeserializeOwned};

use crate::server::memory_strategy::{
    MemoryError, MemoryStrategy, default_strategies::local_storage::definition::LocalMemory,
};

impl MemoryStrategy for LocalMemory {
    fn store_key<T>(&'static self, key: &str, item: &T) -> Result<(), MemoryError>
    where
        T: serde::Serialize + std::fmt::Debug,
    {
        let string =
            serde_json::to_string(item).map_err(|e| MemoryError::Storage(e.to_string()))?;

        let mut map = self
            .storage
            .lock()
            .map_err(|e| MemoryError::Storage(format!("Failed to get lock: {e}")))?;

        map.insert(key.to_string(), string);

        Ok(())
    }

    fn retrieve_key<T>(
        &'static self,
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

    fn modify_key_mut<'a, T, F, RES>(
        &'static self,
        key: &str,
        func: F,
    ) -> Result<Option<RES>, MemoryError>
    where
        T: serde::Deserialize<'a> + std::fmt::Debug + Serialize,
        F: FnOnce(&mut T) -> RES,
    {
        let mut map = self
            .storage
            .lock()
            .map_err(|e| MemoryError::Retrieve(format!("Failed to get lock: {e}")))?;

        match map.remove_entry(key) {
            // SAFETY: In my head this should be fine.
            //
            // We leak `value` to the stack so serde is happy with a static lifetime.
            // We do work in func()
            // We re-encode the modified value back to json.
            // We save the json back to the map.
            // We take the static pointer and cast it back to a box and drop it
            //
            // This could technically break if in func(), the user returns the `&mut` pointer out as `RET`, but I think the rust borrowchecker should catch that.
            Some((key, value)) => unsafe {
                let mut value: &'static str = Box::leak(Box::new(value));

                let mut item: T = serde_json::from_str(value)
                    .map_err(|e| MemoryError::Retrieve(e.to_string()))?;

                let result = func(&mut item);

                let json = serde_json::to_string(&item)
                    .map_err(|e| MemoryError::Storage(e.to_string()))?;

                map.insert(key, json);

                drop(Box::from_raw(&mut value));

                Ok(Some(result))
            },

            None => Ok(None),
        }
    }
}
