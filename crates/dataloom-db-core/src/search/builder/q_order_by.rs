use crate::search::{
    builder::SearchQueryBuilder,
    table_options::{TableOptionsValue, order_by_options::OrderByOptions},
};

impl SearchQueryBuilder {
    pub fn q_order_by(
        mut self,
        column: impl Into<String>,
        options: impl Into<Option<OrderByOptions>>,
    ) -> Self {
        self.table_options.insert(TableOptionsValue::OrderBy {
            column: column.into(),
            options: options.into(),
        });
        self
    }
}
