use crate::core::search::builder::SearchQueryBuilder;

impl SearchQueryBuilder {
    pub fn new() -> Self {
        Self {
            constraint: None,
            table_options: None,
        }
    }
}

impl Default for SearchQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}
