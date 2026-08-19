use crate::core::search::{builder::SearchQueryBuilder, selection_options::SelectionOptions};

impl SearchQueryBuilder {
    pub fn add_select_options(mut self, options: SelectionOptions) -> Self {
        self.select_options = Some(options);

        self
    }
}
