use chrono::{DateTime, Utc};

use crate::core::column::ColumnValue;

impl From<DateTime<Utc>> for ColumnValue {
    fn from(value: DateTime<Utc>) -> Self {
        ColumnValue::Date(value)
    }
}
