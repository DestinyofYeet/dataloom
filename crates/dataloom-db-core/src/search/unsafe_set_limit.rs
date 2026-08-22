use crate::search::{SearchQuery, table_options::TableOptions};

impl SearchQuery {
    /// # Safety
    ///
    /// This is not actually unsafe, just a deterrent to use this function.
    pub unsafe fn set_limit(&mut self, limit: u64) {
        self.table_options = Some(match self.table_options.take() {
            Some(value) => value.limit(limit),
            None => TableOptions::new().limit(limit),
        });
    }
}
