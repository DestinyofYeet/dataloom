use dataloom_db_core::{
    search::{SearchQuery, table_options::order_by_options::OrderByOptions},
    traits::DatabaseStrategy,
};

use crate::tests::{TestData, TestModel, setup_test_server};

#[test]
pub fn test_order_by() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let conn = db.get_connection();

    let mut model1 = TestModel::new("model1", 0, TestData::One("hi".to_string()));

    db.save_model(&conn, &mut model1).unwrap();

    let mut model2 = TestModel::new("model2", 1, TestData::Two);

    db.save_model(&conn, &mut model2).unwrap();

    let retrieved = db
        .search_single_model::<TestModel>(
            &conn,
            SearchQuery::builder()
                .q_order_by("number", OrderByOptions::Desc)
                .build(),
        )
        .unwrap()
        .unwrap();

    assert_eq!(retrieved, model2);
}
