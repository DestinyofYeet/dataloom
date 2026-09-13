use crate::{
    server::memory_strategy::MemoryStrategy,
    tests::{TestData, TestModel, setup_test_server},
};

#[test]
fn update_empty_init() {
    let server = setup_test_server();

    let memory = server.get_memory();

    assert!(memory.get::<TestModel>().unwrap().is_none());

    memory
        .update::<TestModel, _, _>(|value| {
            *value = Some(TestModel::new("blub", 123, TestData::Two));
        })
        .unwrap();

    assert!(memory.get::<TestModel>().unwrap().is_some());
}
