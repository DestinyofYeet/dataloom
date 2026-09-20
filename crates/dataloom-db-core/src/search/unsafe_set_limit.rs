use crate::search::{SearchQuery, table_options::TableOptionsValue};

impl SearchQuery {
    /// # Safety
    ///
    /// This is not actually unsafe, just a deterrent to use this function.
    pub unsafe fn set_limit(&mut self, limit: u64) {
        self.table_options.insert(TableOptionsValue::Limit(limit));
    }
}
