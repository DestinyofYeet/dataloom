use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use dataloom_macro::{FromIter, SaveData};
use serde::{Deserialize, Serialize};

use crate::{
    self as dataloom,
    core::{
        MigrationKind, ModelMigration,
        column::{
            ColumnType, ModifyColumn, ModifyColumnOptionsValues,
            create::{CreateColumn, CreateOptions},
        },
        traits::model::Model,
    },
};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq)]
pub enum TestData {
    One(String),
    Two,
}

#[derive(Debug, Clone, SaveData, FromIter, Serialize, Deserialize, PartialEq)]
pub struct TestModel {
    pub id: Option<i64>,
    pub name: String,
    pub number: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub extra_data: TestData,
}

impl TestModel {
    pub fn new(
        name: impl Into<String>,
        number: impl Into<Option<i64>>,
        extra_data: TestData,
    ) -> Self {
        Self {
            id: None,
            name: name.into(),
            // This model uses 0 for the default. If `None` is passed in, it is set to `Some(0)` in order for `assert_eq`s to pass.
            number: Some(number.into().unwrap_or(0)),
            created_at: Utc::now(),
            extra_data,
        }
    }
}

impl Model for TestModel {
    const TABLE_NAME: &'static str = "TestModel";

    fn get_migration() -> &'static Vec<crate::core::ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![
                ModelMigration::new(
                    0,
                    MigrationKind::Create(vec![
                        CreateColumn::new(
                            "id",
                            ColumnType::Integer,
                            CreateOptions::default().set_primary_key(),
                        ),
                        CreateColumn::new(
                            "name",
                            ColumnType::String,
                            CreateOptions::default().set_non_nullable(),
                        ),
                        CreateColumn::new(
                            "created_at",
                            ColumnType::Date,
                            CreateOptions::default().set_non_nullable(),
                        ),
                        CreateColumn::new(
                            "extra_data",
                            ColumnType::Json,
                            CreateOptions::default().set_non_nullable(),
                        ),
                    ]),
                ),
                ModelMigration::new(
                    1,
                    MigrationKind::Modify(vec![ModifyColumn::new(
                        "number",
                        ModifyColumnOptionsValues::Add {
                            new_type: ColumnType::Integer,
                            new_options: CreateOptions::default().set_default("0".to_string()),
                        },
                    )]),
                ),
            ]
        });

        &MIGRATIONS
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }
}
