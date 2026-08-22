use crate::search::{builder::SearchQueryBuilder, table_options::TableOptions};

impl SearchQueryBuilder {
    pub fn table_options(mut self, options: impl Into<TableOptions>) -> Self {
        self.table_options = Some(options.into());

        self
    }
}
