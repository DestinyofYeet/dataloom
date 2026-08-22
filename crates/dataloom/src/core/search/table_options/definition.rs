use std::collections::HashSet;

use crate::core::search::table_options::table_options_value::TableOptionsValue;

#[derive(Debug, Clone)]
pub struct TableOptions {
    pub(super) options: HashSet<TableOptionsValue>,
}
