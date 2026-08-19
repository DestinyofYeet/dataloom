use crate::core::search::table_options::table_options_value::TableOptionsValue;

impl TableOptionsValue {
    pub fn priority(&self) -> u8 {
        match self {
            TableOptionsValue::Limit(_) => 1,
            TableOptionsValue::OrderBy {
                column: _,
                options: _,
            } => 10,
        }
    }
}
