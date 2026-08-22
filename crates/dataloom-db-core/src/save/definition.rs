use crate::column::ColumnValue;

#[derive(Debug)]
pub struct SaveModel {
    pub key: String,
    pub value: ColumnValue,
}

impl SaveModel {
    pub fn new<T>(key: impl ToString, value: T) -> Self
    where
        T: Into<ColumnValue>,
    {
        Self {
            key: key.to_string(),
            value: value.into(),
        }
    }
}
