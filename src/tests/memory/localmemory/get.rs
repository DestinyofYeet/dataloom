use chrono::Utc;

use crate::{
    server::memory_strategy::{MemoryError, MemoryStrategy},
    tests::{TestData, TestModel, setup_test_server},
};

#[test]
fn store() {
    let server = setup_test_server();

    let memory = server.get_memory();

    let data = TestModel {
        id: None,
        name: "blub".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("whoop".to_string()),
    };

    memory.store(&data).expect("to save data");
}

#[test]
fn get() {
    let server = setup_test_server();

    let memory = server.get_memory();

    let data = TestModel {
        id: None,
        name: "blub".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("whoop".to_string()),
    };

    memory.store(&data).expect("to save data");

    let ret_data = memory
        .retrieve::<TestModel>()
        .expect("to get data")
        .expect("to have data");

    assert_eq!(data, ret_data);
}

#[test]
fn update() {
    let server = setup_test_server();

    let memory = server.get_memory();

    let mut data = TestModel {
        id: None,
        name: "blub".to_string(),
        created_at: Utc::now(),
        extra_data: TestData::One("whoop".to_string()),
    };

    memory.store(&data).expect("to update data");

    // for _ in 0..50_000_000 {
    let _: Result<(), MemoryError> = memory.modify::<TestModel, _, _>(|item| {
        if let Some(item) = item {
            item.name = "rofl".to_string()
        }
    });
    // }

    data.name = "rofl".to_string();

    let ret_data = memory
        .retrieve::<TestModel>()
        .expect("to get data")
        .expect("to have data");

    assert_eq!(data, ret_data);
}
