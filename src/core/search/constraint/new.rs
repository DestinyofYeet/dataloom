use crate::core::{
    column::ColumnValue,
    search::{constraint::SearchConstraint, search_op::SearchOp},
};

impl SearchConstraint {
    pub fn new(column: impl Into<String>, operator: SearchOp, value: ColumnValue) -> Self {
        Self {
            column: column.into(),
            operator,
            value,
            other: None,
        }
    }
}
