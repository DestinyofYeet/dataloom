use crate::core::search::{constraint::SearchConstraint, table_options::TableOptions};

pub struct SearchQueryBuilder {
    pub(super) constraint: Option<SearchConstraint>,
    pub(super) table_options: Option<TableOptions>,
}
