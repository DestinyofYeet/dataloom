use std::collections::HashSet;

use crate::core::search::table_options::TableOptions;

impl TableOptions {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }
}

impl Default for TableOptions {
    fn default() -> Self {
        Self::new()
    }
}
