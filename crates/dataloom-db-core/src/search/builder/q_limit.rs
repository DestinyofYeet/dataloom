use crate::search::{builder::SearchQueryBuilder, table_options::TableOptionsValue};

impl SearchQueryBuilder {
    pub fn q_limit(mut self, limit: u64) -> Self {
        self.table_options.insert(TableOptionsValue::Limit(limit));
        self
    }
}
