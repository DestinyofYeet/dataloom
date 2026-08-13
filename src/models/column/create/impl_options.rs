use tracing::error;

use crate::models::column::{
    ColumnType,
    create::{CreateColumnOptionsValues, CreateOptions, CreateTableOptionValues},
};

impl CreateOptions {
    /// This column can be null (default: false)
    pub fn set_non_nullable(mut self) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::NonNullable));

        self
    }

    /// This column should be a primary key (default: false)
    /// This implies `set_nullable()`
    /// The type of this column should be `Integer`
    pub fn set_primary_key(mut self) -> Self {
        self = self.set_non_nullable();
        self.column_options
            .insert((0, CreateColumnOptionsValues::PrimaryKey));

        self
    }

    /// This column should have a default value (default: None)
    pub fn set_default(mut self, value: String) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Default(value)));

        self
    }

    /// This column should only have unique values (default: false)
    pub fn set_unique(mut self) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Unique));

        self
    }

    /// This column should only have values that pass a check
    /// If you want to write a check like this
    /// ```sql
    /// CREATE TABLE test (
    ///   value INT CHECK(value > 0)
    /// )
    /// ```
    /// this needs to be called like `set_check("value > 0")`
    pub fn set_check(mut self, value: String) -> Self {
        self.column_options
            .insert((0, CreateColumnOptionsValues::Check(value)));

        self
    }

    /// This column will reference a foreign key
    pub fn set_foreign_key(mut self, table: impl ToString, column: impl ToString) -> Self {
        self.table_options
            .insert(CreateTableOptionValues::ForeignKey {
                table: table.to_string(),
                column: column.to_string(),
            });

        self
    }
}
