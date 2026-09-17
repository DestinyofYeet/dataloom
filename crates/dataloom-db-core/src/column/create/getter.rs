use crate::column::create::{CreateColumn, CreateColumnOptionsValues, CreateOptions};

impl CreateOptions {
    pub fn is_optional(&self) -> bool {
        !self
            .column_options
            .contains(&(0, CreateColumnOptionsValues::NonNullable))
    }
}
