use crate::{column::ColumnType, traits::DatabaseStrategyError};

pub struct FromIterValue {
    pub column_name: String,
    pub column_value: String,
    pub column_type: ColumnType,
}

pub trait FromIter {
    fn from_iter(iter: impl Iterator<Item = FromIterValue>) -> Result<Self, DatabaseStrategyError>
    where
        Self: Sized;
}
