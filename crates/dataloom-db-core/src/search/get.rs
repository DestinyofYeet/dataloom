use std::collections::HashSet;

use crate::search::{
    SearchQuery, constraint::SearchConstraint, join_options::JoinOptions,
    table_options::TableOptionsValue,
};

impl SearchQuery {
    #[inline]
    pub fn values(
        self,
    ) -> (
        Option<SearchConstraint>,
        HashSet<TableOptionsValue>,
        Vec<JoinOptions>,
    ) {
        (self.constraint, self.table_options, self.join_options)
    }
}
