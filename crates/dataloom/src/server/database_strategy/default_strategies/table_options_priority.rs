use crate::{
    core::search::table_options::table_options_value::TableOptionsValue,
    server::database_strategy::default_strategies::SqliteStrategy,
};

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
