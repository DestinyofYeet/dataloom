use std::path::PathBuf;

pub mod memory;
mod test_model;
pub use test_model::*;
use tracing::trace;
use tracing_subscriber::EnvFilter;

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
    trace!("tempdir is: {tempfile:?}");
    tempfile.keep()
}

pub fn setup_server<D, M>(strategy: D, memory: M) -> DataloomServer<D, M>
where
    D: DatabaseStrategy + 'static,
    M: MemoryStrategy + 'static,
{
    DataloomServer::new(1, TracingStrategy {}, strategy, memory).expect("to create server")
}

pub fn setup_test_server() -> DataloomServer<SqliteStrategy, LocalMemory> {
    _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("trace"))
        .with_test_writer()
        .try_init();

    let dir = get_test_dir();
    setup_server(
        SqliteStrategy::new(dir.join("database.db").to_str().unwrap()),
        LocalMemory::new(),
    )
}
