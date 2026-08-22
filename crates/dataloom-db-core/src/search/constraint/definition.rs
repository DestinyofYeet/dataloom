use crate::{column::ColumnValue, search::search_op::SearchOp};

#[derive(Debug, Clone)]
pub enum OtherConstraint {
    And(SearchConstraint),
    Or(SearchConstraint),
}

#[derive(Debug, Clone)]
pub struct SearchConstraint {
    pub(super) column: String,
    pub(super) operator: SearchOp,
    pub(super) value: ColumnValue,
    pub(super) other: Option<Box<OtherConstraint>>,
}
