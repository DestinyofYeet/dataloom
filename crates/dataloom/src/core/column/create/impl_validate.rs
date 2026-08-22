use tracing::error;

use crate::core::column::{
    ColumnType,
    create::{CreateColumnOptionsValues, CreateOptions},
};

impl CreateOptions {
    pub(crate) fn validate(&self, column_name: &str, column_type: ColumnType) -> bool {
        let mut is_ok = true;

        let mut mk_error = |msg: String| {
            is_ok = false;
            error!("Failed to validate create options for {column_name}: {msg}");
        };

        for (_, option) in self.column_options.iter() {
            match option {
                CreateColumnOptionsValues::NonNullable => {}
                CreateColumnOptionsValues::PrimaryKey => {
                    if column_type != ColumnType::Integer {
                        mk_error(format!(
                            "Column must be an integer (but it has type {column_type:?}) because it was selected as a primary key!",
                        ))
                    }
                }
                CreateColumnOptionsValues::Default(default) => match column_type {
                    ColumnType::String => {}
                    ColumnType::Integer => {
                        if let Err(e) = default.parse::<i64>() {
                            mk_error(format!(
                                "Default value '{default}' does not parse into a i64: {e}"
                            ))
                        }
                    }
                    ColumnType::Float => {
                        if let Err(e) = default.parse::<f64>() {
                            mk_error(format!(
                                "Default value '{default}' does not parse into a f64: {e}"
                            ))
                        }
                    }
                    ColumnType::Date => {}
                    ColumnType::Json => {}
                    ColumnType::Bool => {}
                },
                CreateColumnOptionsValues::Unique => {}
                CreateColumnOptionsValues::Check(_) => {}
            }
        }

        is_ok
    }
}
