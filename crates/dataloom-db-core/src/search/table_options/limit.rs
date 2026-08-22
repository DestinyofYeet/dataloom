use crate::search::table_options::{TableOptions, table_options_value::TableOptionsValue};

impl TableOptions {
    pub fn limit(mut self, limit: u64) -> Self {
        self.options.insert(TableOptionsValue::Limit(limit));

        self
    }
}
