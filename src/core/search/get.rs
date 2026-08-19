use crate::core::search::{SearchQuery, constraint::SearchConstraint, table_options::TableOptions};

impl SearchQuery {
    #[inline]
    pub fn values(self) -> (Option<SearchConstraint>, Option<TableOptions>) {
        (self.constraint, self.table_options)
    }
}
