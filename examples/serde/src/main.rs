use std::sync::LazyLock;

use dataloom::dataloom_db_core::column::FromColumn;
use dataloom::dataloom_db_core::column::create::{CreateColumn, CreateOptions};
use dataloom::dataloom_db_core::search::SearchQuery;
use dataloom::dataloom_db_core::traits::from_iter::{FromIter, FromIterValue};
use dataloom::dataloom_db_core::traits::save_data::SaveData;
use dataloom::dataloom_db_core::traits::{DatabaseStrategy, DatabaseStrategyError};
use dataloom::dataloom_db_core::{MigrationKind, ModelMigration};
use dataloom::dataloom_db_sqlite::SqliteStrategy;
use dataloom::dataloom_macro::{FromIter, SaveData};
use dataloom::server::memory_strategy::default_strategies::local_storage::LocalMemory;
use dataloom::{
    dataloom_db_core::{column::ColumnType, traits::model::Model},
    server::DataloomServer,
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestTest {
    key: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestEnum {
    Id(i32),
}

#[derive(Debug, Serialize, Deserialize, SaveData)]
pub struct Test {
    id: Option<i64>,
    key: String,
    value: i32,
    test_test: TestTest,
    test_enum: TestEnum,
}

impl Model for Test {
    const TABLE_NAME: &'static str = "test";

    fn get_migration() -> &'static Vec<ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new("key", ColumnType::String, CreateOptions::default()),
                    CreateColumn::new("value", ColumnType::Integer, CreateOptions::default()),
                    CreateColumn::new("test_test", ColumnType::Json, CreateOptions::default()),
                    CreateColumn::new("test_enum", ColumnType::Json, CreateOptions::default()),
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

impl FromIter for Test {
    fn from_iter(iter: impl Iterator<Item = FromIterValue>) -> Result<Self, DatabaseStrategyError>
    where
        Self: Sized,
    {
        let mut id: Option<i64> = None;
        let mut key: Option<String> = None;
        let mut value: Option<i32> = None;
        let mut test_test: Option<TestTest> = None;
        let mut test_enum: Option<TestEnum> = None;

        for FromIterValue {
            column_name,
            column_value,
            column_type,
        } in iter
        {
            if column_name == Self::get_latest_column_name("id").unwrap() {
                id = column_value
                    .from_column(column_type)
                    .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
            }

            if column_name == Self::get_latest_column_name("key").unwrap() {
                key = column_value
                    .from_column(column_type)
                    .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
            }

            if column_name == Self::get_latest_column_name("value").unwrap() {
                value = column_value
                    .from_column(column_type)
                    .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
            }

            if column_name == Self::get_latest_column_name("test_test").unwrap() {
                test_test = column_value
                    .from_column(column_type)
                    .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
            }

            if column_name == Self::get_latest_column_name("test_enum").unwrap() {
                test_enum = column_value
                    .from_column(column_type)
                    .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
            }
        }

        Ok(Self {
            id: Some(id.unwrap()),
            key: key.unwrap(),
            value: value.unwrap(),
            test_test: test_test.unwrap(),
            test_enum: test_enum.unwrap(),
        })
    }
}

// impl SaveData for Test {
//     fn get_save_data(&self) -> Vec<SaveModel> {
//         vec![
//             SaveModel::new("id", self.id.to_column().unwrap()),
//             SaveModel::new("key", self.key.to_column().unwrap()),
//             SaveModel::new("value", self.value.to_column().unwrap()),
//             SaveModel::new("test_test", self.test_test.to_column().unwrap()),
//             SaveModel::new("test_enum", self.test_enum.to_column().unwrap()),
//         ]
//     }
// }

fn main() {
    let mut server = DataloomServer::new(
        8,
        TracingStrategy {},
        SqliteStrategy::new("./test.db"),
        // SqliteStrategy::new_memory(),
        LocalMemory::new(),
    )
    .unwrap();

    let db = server.get_database();
    db.migrate_model::<Test>().unwrap();

    // let mut test = Test {
    //     id: None,
    //     key: "test".to_string(),
    //     value: 8,
    //     test_test: TestTest { key: 8 },
    //     test_enum: TestEnum::Id(9),
    // };

    // dbg!(test.get_save_data());

    // db.save_model(&db.get_connection(), &mut test).unwrap();

    let model = db
        .search_single_model::<Test>(
            &db.get_connection(),
            SearchQuery::builder().add_constraint(("id", 1)).build(),
        )
        .unwrap();
    // .unwrap();

    dbg!(model);

    // dbg!(model.get_save_data());

    server.shutdown().unwrap()
}
