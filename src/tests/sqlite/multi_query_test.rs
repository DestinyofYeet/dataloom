use dataloom_db_core::{
    column::ToColumn,
    search::{SearchQuery, constraint::SearchConstraint, search_op::SearchOp},
    traits::DatabaseStrategy,
};

use crate::tests::{TestData, TestModel, setup_test_server};

#[test]
pub fn multi_query_test() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let conn = db.get_connection();

    let mut model = TestModel::new("some_name", 8, TestData::One("weee".to_string()));

    let mut model2 = TestModel::new("some_name", 1, TestData::Two);

    db.save_model(&conn, &mut model).unwrap();
    db.save_model(&conn, &mut model2).unwrap();

    let retrieved = db
        .search_single_model::<TestModel>(
            &conn,
            SearchQuery::builder()
                .q_where(
                    SearchConstraint::new::<TestModel>("name", SearchOp::EQ, "some_name")
                        .unwrap()
                        .and(
                            SearchConstraint::new::<TestModel>(
                                "extra_data",
                                SearchOp::EQ,
                                (TestData::Two).to_column().unwrap(),
                            )
                            .unwrap(),
                        ),
                )
                .build(),
        )
        .unwrap()
        .unwrap();

    dbg!(&retrieved);

    assert_eq!(retrieved, model2);
}
