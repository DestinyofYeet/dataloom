use crate::{
    tasks::{
        default_tasks::database::{GetModelTask, SaveModelTask},
        task::TaskState,
    },
    tests::{TestData, TestModel, setup_test_server},
};
use dataloom_db_core::search::table_options::TableOptions;
use dataloom_db_core::search::table_options::table_options_value::order_by_options::OrderByOptions;
use dataloom_db_core::{column::ToColumn, traits::DatabaseStrategy};

use chrono::Utc;
use dataloom_db_core::search::SearchQuery;
use dataloom_db_core::search::constraint::SearchConstraint;
use dataloom_db_core::search::search_op::SearchOp;

#[test]
pub fn test_save_and_retrieve2() {
    let mut server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::Two,
        number: None,
    };

    db.save_model(&db.get_connection(), &mut model).unwrap();

    db.search_single_model::<TestModel>(
        &db.get_connection(),
        SearchQuery::builder()
            .add_constraint(("id", model.id.unwrap()))
            .build(),
    )
    .unwrap()
    .unwrap();

    server.shutdown().unwrap();
}

#[test]
pub fn test_save_and_retrieve() {
    let mut server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("weeee".to_string()),
        number: None,
    };

    db.save_model(&db.get_connection(), &mut model).unwrap();

    db.search_single_model::<TestModel>(
        &db.get_connection(),
        SearchQuery::builder()
            .add_constraint(("id", model.id.unwrap()))
            .build(),
    )
    .unwrap()
    .unwrap();

    server.shutdown().unwrap();
}

#[test]
pub fn test_save_and_retrieve_task() {
    let mut server = setup_test_server();
    let task_handler = server.get_task_handler();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel::new("some_name", None, TestData::One("weee".to_string()));

    let save_task = SaveModelTask::new(model.clone());

    let task = task_handler.spawn_task(save_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    let result = task.get_result();

    let get_task = GetModelTask::<TestModel>::new(
        SearchQuery::builder()
            .add_constraint(("id", result.unwrap()))
            .build(),
    );

    let task = task_handler.spawn_task(get_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    let result = task.get_result().unwrap().unwrap().unwrap();
    model.id = result.id;

    assert_eq!(model, result);

    server.shutdown().unwrap();
}

#[test]
pub fn multi_query_test() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let conn = db.get_connection();

    let mut model = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("weeee".to_string()),
        number: Some(8),
    };

    let mut model2 = TestModel {
        id: None,
        name: "some_name".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::Two,
        number: Some(1),
    };

    db.save_model(&conn, &mut model).unwrap();
    db.save_model(&conn, &mut model2).unwrap();

    let retrieved = db
        .search_single_model::<TestModel>(
            &conn,
            SearchQuery::builder()
                .add_constraint(
                    SearchConstraint::new("name", SearchOp::EQ, "some_name").and(
                        SearchConstraint::new(
                            "extra_data",
                            SearchOp::EQ,
                            (TestData::Two).to_column().unwrap(),
                        ),
                    ),
                )
                .build(),
        )
        .unwrap()
        .unwrap();

    dbg!(&retrieved);

    assert_eq!(retrieved, model2);
}

#[test]
pub fn test_order_by() {
    let server = setup_test_server();
    let db = server.get_database();

    db.migrate_model::<TestModel>().unwrap();

    let conn = db.get_connection();

    let mut model1 = TestModel {
        id: None,
        name: "model1".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("hi".to_string()),
        number: Some(0),
    };

    db.save_model(&conn, &mut model1).unwrap();

    let mut model2 = TestModel {
        id: None,
        name: "model2".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::Two,
        number: Some(1),
    };

    db.save_model(&conn, &mut model2).unwrap();

    let retrieved = db
        .search_single_model::<TestModel>(
            &conn,
            SearchQuery::builder()
                .table_options(TableOptions::new().order_by("number", OrderByOptions::Desc))
                .build(),
        )
        .unwrap()
        .unwrap();

    assert_eq!(retrieved, model2);
}
