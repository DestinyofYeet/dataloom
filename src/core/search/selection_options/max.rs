use crate::core::search::selection_options::{SelectionOptions, SelectionOptionsValues};

impl SelectionOptions {
    pub fn max(column: impl Into<String>) -> Self {
        Self {
            option: SelectionOptionsValues::Max {
                column: column.into(),
            },
        }
    }
}
