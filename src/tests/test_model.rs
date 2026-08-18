use std::sync::LazyLock;

use chrono::{DateTime, Utc};
use dataloom_macro::{FromIter, SaveData};
use serde::{Deserialize, Serialize};

use crate::{
    self as dataloom,
    models::{
        MigrationKind, ModelMigration,
        column::{
            ColumnType,
            create::{CreateColumn, CreateOptions},
        },
        traits::model::Model,
    },
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TestData {
    One(String),
    Two,
}

#[derive(Debug, SaveData, FromIter, Serialize, Deserialize, PartialEq)]
pub struct TestModel {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub extra_data: TestData,
}

impl Model for TestModel {
    const TABLE_NAME: &'static str = "TestModel";

    fn get_migration() -> &'static Vec<crate::models::ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
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
            )]
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
