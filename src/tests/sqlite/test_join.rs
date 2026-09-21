use std::sync::LazyLock;

use dataloom_db_core::{
    MigrationKind, ModelMigration,
    column::{
        ColumnType,
        create::{CreateColumn, CreateOptions},
    },
    search::{SearchQuery, constraint::SearchConstraint, search_op::SearchOp},
    traits::{DatabaseStrategy, model::Model},
};
use dataloom_macro::{FromIter, SaveData};

use crate::tests::setup_test_server;

#[derive(Debug, Clone, FromIter, SaveData)]
struct Customer {
    id: Option<i64>,
    name: String,
}

impl Model for Customer {
    const TABLE_NAME: &'static str = "Customers";

    fn get_migration() -> &'static Vec<dataloom_db_core::ModelMigration> {
        static DATA: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
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
                ]),
            )]
        });
        &DATA
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}

#[derive(Debug, Clone, FromIter, SaveData)]
struct Transaction {
    id: Option<i64>,
    customer_id: i64,
    amount: u64,
}

impl Model for Transaction {
    const TABLE_NAME: &'static str = "Transactions";

    fn get_migration() -> &'static Vec<ModelMigration> {
        static DATA: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "customer_id",
                        ColumnType::Integer,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "amount",
                        ColumnType::Integer,
                        CreateOptions::default().set_non_nullable(),
                    ),
                ]),
            )]
        });

        &DATA
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id)
    }
}

#[test]
fn test_join() {
    let server = setup_test_server();
    let db = server.get_database();
    db.migrate_model::<Customer>().unwrap();
    db.migrate_model::<Transaction>().unwrap();

    let mut c1 = Customer {
        id: None,
        name: "Customer1".to_string(),
    };
    c1.save(db.clone()).unwrap();

    let mut c2 = Customer {
        id: None,
        name: "Customer2".to_string(),
    };
    c2.save(db.clone()).unwrap();

    for cost in 0..100 {
        let customer = { if cost % 2 == 0 { &c1 } else { &c2 } };
        let mut trx = Transaction {
            id: None,
            customer_id: customer.id.unwrap(),
            amount: cost,
        };

        trx.save(db.clone()).unwrap();
    }

    let results = Transaction::search_multiple(
        db.clone(),
        SearchQuery::builder()
            // = join on Customer.id = Transaction.customer_id
            .q_join::<Customer>("id", "customer_id")
            .q_where(SearchConstraint::new::<Transaction>("amount", SearchOp::GT, 50).unwrap())
            .q_where(SearchConstraint::new::<Customer>("name", SearchOp::EQ, "Customer1").unwrap())
            .build(),
    )
    .unwrap();

    dbg!(results);
}
