use std::{collections::HashMap, sync::Mutex};

use crate::server::memory_strategy::default_strategies::local_storage::definition::LocalMemory;

impl LocalMemory {
    pub fn new() -> Self {
        Self {
            storage: Mutex::new(HashMap::new()),
        }
    }
}

impl Default for LocalMemory {
    fn default() -> Self {
        Self::new()
    }
}
