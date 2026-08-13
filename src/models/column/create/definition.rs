use tracing::error;

use crate::models::column::ColumnType;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CreateColumnOptionsValues {
    NonNullable,
    PrimaryKey,
    Default(String),
    Unique,
    Check(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CreateTableOptionValues {
    ForeignKey { table: String, column: String },
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct CreateOptions {
    pub(crate) column_options: HashSet<(u64, CreateColumnOptionsValues)>,
    pub(crate) table_options: HashSet<CreateTableOptionValues>,
}

pub struct CreateColumn {
    pub(crate) key: String,
    pub(crate) value: ColumnType,
    pub(crate) options: CreateOptions,
}

impl CreateColumn {
    pub fn new(key: impl ToString, value: ColumnType, options: CreateOptions) -> Self {
        let key = key.to_string();

        if !options.validate(&key, value) {
            panic!("Failed to validate options!");
        }

        Self {
            key,
            value,
            options,
        }
    }
}
