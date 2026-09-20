use std::collections::HashSet;

use crate::search::{
    constraint::SearchConstraint, join_options::JoinOptions, table_options::TableOptionsValue,
};

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub(super) constraint: Option<SearchConstraint>,
    pub(super) table_options: HashSet<TableOptionsValue>,
    pub(super) join_options: Vec<JoinOptions>,
}
