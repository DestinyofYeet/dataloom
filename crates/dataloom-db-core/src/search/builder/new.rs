use std::collections::HashSet;

use crate::search::builder::SearchQueryBuilder;

impl SearchQueryBuilder {
    pub fn new() -> Self {
        Self {
            constraint: None,
            table_options: HashSet::new(),
            join_options: Vec::new(),
        }
    }
}

impl Default for SearchQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
