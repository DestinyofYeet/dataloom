use crate::core::search::selection_options::{SelectionOptions, SelectionOptionsValues};

impl SelectionOptions {
    pub fn min(column: impl Into<String>) -> Self {
        Self {
            option: SelectionOptionsValues::Min {
                column: column.into(),
            },
        }
    }
}
