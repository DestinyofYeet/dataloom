use crate::search::{builder::SearchQueryBuilder, constraint::SearchConstraint};

impl SearchQueryBuilder {
    pub fn add_constraint(mut self, constraint: impl Into<SearchConstraint>) -> Self {
        self.constraint = Some(constraint.into());
        self
    }
}
