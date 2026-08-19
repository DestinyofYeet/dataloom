use crate::core::search::table_options::{
    TableOptions,
    table_options_value::{TableOptionsValue, order_by_options::OrderByOptions},
};

impl TableOptions {
    pub fn order_by(
        mut self,
        column: impl Into<String>,
        options: impl Into<Option<OrderByOptions>>,
    ) -> Self {
        self.options.insert(TableOptionsValue::OrderBy {
            column: column.into(),
            options: options.into(),
        });

        self
    }
}
