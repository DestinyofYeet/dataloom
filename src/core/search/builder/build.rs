use crate::core::search::{SearchQuery, builder::SearchQueryBuilder};

impl SearchQueryBuilder {
    pub fn build(self) -> SearchQuery {
        let Self {
            constraint,
            table_options,
        } = self;

        SearchQuery {
            constraint,
            table_options,
        }
    }
}
