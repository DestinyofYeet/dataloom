use crate::core::search::{SearchQuery, builder::SearchQueryBuilder};

impl SearchQuery {
    pub fn builder() -> SearchQueryBuilder {
        SearchQueryBuilder::new()
    }
}
