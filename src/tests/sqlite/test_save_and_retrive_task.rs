use dataloom_db_core::{
    search::{SearchQuery, constraint::SearchConstraint, search_op::SearchOp},
    traits::DatabaseStrategy,
};

use crate::{
    tasks::{
        default_tasks::database::{GetModelTask, SaveModelTask},
        task::TaskState,
    },
    tests::{TestData, TestModel, setup_test_server},
};

#[test]
pub fn test_save_and_retrieve_task() {
    let server = setup_test_server();
    let task_handler = server.get_task_handler();
    let db = server.get_database();

    println!("hi");

    db.migrate_model::<TestModel>().unwrap();

    let mut model = TestModel::new("some_name", None, TestData::One("weee".to_string()));

    let save_task = SaveModelTask::new(model.clone());

    let task = task_handler.spawn_task(save_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    let result = task.get_result().unwrap();

    let get_task = GetModelTask::<TestModel>::new(
        SearchQuery::builder()
            .q_where(
                SearchConstraint::new::<TestModel>("id", SearchOp::EQ, result.unwrap()).unwrap(),
            )
            .build(),
    );

    let task = task_handler.spawn_task(get_task).unwrap();

    task_handler.wait_until_done(&task).unwrap();

    assert_eq!(task.get_state(), TaskState::Done);

    let result = task.get_result().unwrap().unwrap().unwrap();
    model.id = result.id;

    assert_eq!(model, result);
}
