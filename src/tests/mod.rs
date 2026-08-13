use std::path::PathBuf;

use crate::{
    server::{
        DataloomServer,
        database_strategy::{DatabaseStrategy, default_strategies::SqliteStrategy},
        memory_strategy::{MemoryStrategy, default_strategies::local_storage::LocalMemory},
    },
    tasks::logstrategy::default_strategies::tracing_strategy::TracingStrategy,
};

mod example;
mod sqlite;

fn get_test_dir() -> PathBuf {
    let tempfile = tempfile::TempDir::new().expect("to get temp dir");
    tempfile.keep()
}

pub fn setup_server<D, M>(strategy: D, memory: M) -> DataloomServer<D, M>
where
    D: DatabaseStrategy + 'static,
    M: MemoryStrategy + 'static,
{
    DataloomServer::new(1, TracingStrategy {}, strategy, memory).expect("to create server")
}

pub fn setup_sqlite_server() -> DataloomServer<SqliteStrategy, LocalMemory> {
    let dir = get_test_dir();
    setup_server(
        SqliteStrategy::new(dir.join("database.db").to_str().unwrap()),
        LocalMemory::new(),
    )
}
