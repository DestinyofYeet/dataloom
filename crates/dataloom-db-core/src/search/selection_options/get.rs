use crate::core::search::selection_options::{SelectionOptions, SelectionOptionsValues};

impl SelectionOptions {
    pub fn get(self) -> SelectionOptionsValues {
        self.option
    }
}
