use dataloom_db_core::{
    search::{SearchQuery, constraint::SearchConstraint, search_op::SearchOp},
    traits::DatabaseStrategy,
};

use crate::tests::{TestData, TestModel, setup_test_server};

#[test]
pub fn test_save_and_retrieve() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel::new("some_name", None, TestData::One("weee".to_string()));

    db.save_model(&db.get_connection(), &mut model).unwrap();

    db.search_single_model::<TestModel>(
        &db.get_connection(),
        SearchQuery::builder()
            .q_where(
                SearchConstraint::new::<TestModel>("id", SearchOp::EQ, model.id.unwrap()).unwrap(),
            )
            .build(),
    )
    .unwrap()
    .unwrap();
}

#[test]
pub fn test_save_and_retrieve2() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel::new("some_name", None, TestData::Two);

    db.save_model(&db.get_connection(), &mut model).unwrap();

    db.search_single_model::<TestModel>(
        &db.get_connection(),
        SearchQuery::builder()
            .q_where(
                SearchConstraint::new::<TestModel>("id", SearchOp::EQ, model.id.unwrap()).unwrap(),
            )
            .build(),
    )
    .unwrap()
    .unwrap();
}
