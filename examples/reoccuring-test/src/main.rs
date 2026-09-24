use std::time::Duration;

use dataloom::{
    dataloom_db_core::traits::DatabaseStrategy,
    dataloom_db_sqlite::SqliteStrategy,
    server::{
        DataloomServer,
        memory_strategy::{MemoryStrategy, default_strategies::local_storage::LocalMemory},
    },
    tasks::{
        logstrategy::default_strategies::tracing_strategy::TracingStrategy,
        reoccurring_tasks::ReoccuringTask, taskrunnable::TaskRunnable,
    },
};
use tracing_subscriber::filter::LevelFilter;

struct ReoccuringTest {
    data: i32,
    name: String,
}

impl<D, ME> TaskRunnable<D, ME> for ReoccuringTest
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn run(
        &mut self,
        info: dataloom::tasks::runnable_info::RunnableInfo<D, ME>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let logger = info.get_logger();

        self.data += 1;

        logger.info(&format!("{}: {}", self.name, self.data));

        Box::new(())
    }
}

fn main() -> Result<(), anyhow::Error> {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env()?;

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let server = DataloomServer::new(
        None,
        TracingStrategy {},
        SqliteStrategy::new_memory(),
        LocalMemory::new(),
    )?;

    let task_handler = server.get_task_handler();

    task_handler.spawn_reocurring_task(ReoccuringTask::new(
        "Task1",
        "*/10 * * * * *",
        ReoccuringTest {
            data: 0,
            name: "1".to_string(),
        },
    )?)?;

    task_handler.spawn_reocurring_task(
        ReoccuringTask::new(
            "Task2",
            "*/15 * * * * *",
            ReoccuringTest {
                data: 100,
                name: "2".to_string(),
            },
        )?
        .run_at_startup(true),
    )?;

    std::thread::sleep(Duration::from_mins(60));

    Ok(())
}
