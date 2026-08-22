use std::{collections::HashMap, sync::Mutex};

pub struct LocalMemory {
    pub(super) storage: Mutex<HashMap<String, String>>,
}
