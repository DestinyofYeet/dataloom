use either::Either;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use thiserror::Error;
use tracing::trace;

use crate::core::column::{ColumnType, ColumnValue};

#[derive(Error, Debug, Deserialize)]
pub enum SerdeColumnError {
    #[error("Failed to serialize: {0}")]
    Serialize(String),

    #[error("Failed to deserialize: {0}")]
    Deserialize(String),
}

impl From<serde_json::Error> for SerdeColumnError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value.to_string())
    }
}

pub trait ToColumn {
    fn to_column(&self) -> Result<ColumnValue, SerdeColumnError>;
}

impl<T> ToColumn for T
where
    T: Serialize,
{
    fn to_column(&self) -> Result<ColumnValue, SerdeColumnError> {
        let value = serde_json::to_value(self)?;

        Ok(match &value {
            serde_json::Value::Null => ColumnValue::Null,
            serde_json::Value::Bool(value) => {
                let value = if *value { 1 } else { 0 };

                ColumnValue::Integer(value)
            }
            serde_json::Value::Number(number) => {
                if number.to_string().contains(".") {
                    ColumnValue::Float(number.as_f64().unwrap())
                } else {
                    ColumnValue::Integer(number.as_i64().unwrap())
                }
            }
            serde_json::Value::String(string) => ColumnValue::String(string.clone()),
            serde_json::Value::Array(_) => ColumnValue::Json(value.to_string()),
            serde_json::Value::Object(_) => ColumnValue::Json(value.to_string()),
        })
    }
}

#[allow(clippy::wrong_self_convention)]
pub trait FromColumn<S> {
    fn from_column<T>(&self, column_type: ColumnType) -> Result<T, SerdeColumnError>
    where
        T: for<'a> Deserialize<'a> + std::fmt::Debug;
}

impl<S> FromColumn<S> for S
where
    Self: ToString,
{
    fn from_column<T>(&self, column_type: ColumnType) -> Result<T, SerdeColumnError>
    where
        T: for<'a> Deserialize<'a> + std::fmt::Debug,
    {
        let value: Result<Either<T, Value>, SerdeColumnError> = match column_type {
            ColumnType::String => Ok(Either::Right(Value::String(self.to_string()))),

            ColumnType::Json => match serde_json::from_str::<T>(&self.to_string()) {
                Ok(value) => Ok(Either::Left(value)),
                Err(e) => {
                    trace!("Failed to parse T from_str: {e} | Trying Value::String");

                    // This is for basic enums with no data
                    Ok(Either::Right(Value::String(self.to_string())))
                }
            },

            ColumnType::Integer => {
                let int: i64 = self
                    .to_string()
                    .parse()
                    .map_err(|e: std::num::ParseIntError| {
                        SerdeColumnError::Deserialize(e.to_string())
                    })?;

                Ok(Either::Right(Value::Number(int.into())))
            }
            ColumnType::Float => {
                let float: f64 =
                    self.to_string()
                        .parse()
                        .map_err(|e: std::num::ParseFloatError| {
                            SerdeColumnError::Deserialize(e.to_string())
                        })?;

                Ok(Either::Right(Value::Number(
                    Number::from_f64(float).unwrap(),
                )))
            }
            ColumnType::Date => Ok(Either::Right(Value::String(self.to_string()))),
            ColumnType::Bool => {
                let value = self.to_string() == "1";

                Ok(Either::Right(Value::Bool(value)))
            }
        };

        trace!(
            "value of column '{}' with type '{column_type:?}': {value:?}",
            self.to_string()
        );

        match value? {
            Either::Left(left) => Ok(left),
            Either::Right(right) => {
                let parsed = serde_json::from_value::<T>(right);

                trace!("parsed into {}: {parsed:?}", std::any::type_name::<T>());

                Ok(parsed?)
            }
        }
    }
}
