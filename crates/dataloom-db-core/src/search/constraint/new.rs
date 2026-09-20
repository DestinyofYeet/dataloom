use std::any::type_name;

use crate::{
    column::ColumnValue,
    search::{constraint::SearchConstraint, search_op::SearchOp},
    traits::{DatabaseStrategyError, model::Model},
};

impl SearchConstraint {
    pub fn new<M>(
        column: impl Into<String>,
        operator: SearchOp,
        value: impl Into<ColumnValue>,
    ) -> Result<Self, DatabaseStrategyError>
    where
        M: Model,
    {
        let column = column.into();
        if M::get_latest_column_name(&column).is_none() {
            return Err(DatabaseStrategyError::Error(format!(
                "Column '{column}' should exist on Model '{}'",
                type_name::<M>()
            )));
        }
        Ok(Self {
            table: M::TABLE_NAME.to_string(),
            column,
            operator,
            value: value.into(),
            other: None,
        })
    }
}
