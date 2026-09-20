use dataloom_db_core::{
    search::{SearchQuery, constraint::SearchConstraint, search_op::SearchOp},
    traits::{DatabaseStrategy, model::Model},
};

use crate::tests::{TestData, TestModel, setup_test_server};

#[test]
pub fn test_optional_field_str() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model1 = TestModel::new("some_name", None, TestData::Two);

    model1.blub = None;

    model1.save(db.clone()).unwrap();

    let retrieved = TestModel::search_single(
        db.clone(),
        SearchQuery::builder()
            .q_where(
                SearchConstraint::new::<TestModel>("id", SearchOp::EQ, model1.id.unwrap()).unwrap(),
            )
            .build(),
    )
    .unwrap()
    .unwrap();

    assert_eq!(model1, retrieved);
}
