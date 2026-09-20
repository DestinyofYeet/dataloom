use crate::search::table_options::order_by_options::OrderByOptions;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TableOptionsValue {
    Limit(u64),
    OrderBy {
        column: String,
        options: Option<OrderByOptions>,
    },
}
