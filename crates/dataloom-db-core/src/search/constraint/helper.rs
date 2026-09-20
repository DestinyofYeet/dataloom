use crate::{
    column::ColumnValue,
    search::{constraint::SearchConstraint, search_op::SearchOp},
    traits::{DatabaseStrategyError, model::Model},
};

impl SearchConstraint {
    pub fn mk_eq<M, T>(column: impl Into<String>, value: T) -> Result<Self, DatabaseStrategyError>
    where
        M: Model,
        T: Into<ColumnValue>,
    {
        Self::new::<M>(column, SearchOp::EQ, value)
    }
}
