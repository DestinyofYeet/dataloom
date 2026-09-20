use dataloom_db_core::search::table_options::TableOptionsValue;

use crate::SqliteStrategy;

impl SqliteStrategy {
    pub(super) fn table_options_priority(value: &TableOptionsValue) -> u64 {
        match value {
            TableOptionsValue::Limit(_) => 10,
            TableOptionsValue::OrderBy {
                column: _,
                options: _,
            } => 1,
        }
    }
}
