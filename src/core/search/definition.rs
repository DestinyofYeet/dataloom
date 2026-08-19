use crate::core::search::{constraint::SearchConstraint, table_options::TableOptions};

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub(super) constraint: Option<SearchConstraint>,
    pub(super) table_options: Option<TableOptions>,
}
