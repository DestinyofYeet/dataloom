use std::collections::HashSet;

use crate::search::table_options::{TableOptions, table_options_value::TableOptionsValue};

impl TableOptions {
    pub fn values(self) -> HashSet<TableOptionsValue> {
        self.options
    }
}
