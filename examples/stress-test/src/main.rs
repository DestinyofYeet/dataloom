use std::time::{Duration, Instant};

use dataloom::{
    dataloom_db_core::traits::DatabaseStrategy,
    dataloom_db_sqlite::SqliteStrategy,
    server::{
        DataloomServer,
        memory_strategy::{MemoryStrategy, default_strategies::local_storage::LocalMemory},
    },
    tasks::{
        logstrategy::default_strategies::tracing_strategy::TracingStrategy,
        task::TaskResult,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};
use rand::random_range;
use tracing_subscriber::filter::LevelFilter;

struct DelayTask {}

impl<D, ME> TaskRunnable<D, ME> for DelayTask
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn run(
        &mut self,
        _: dataloom::tasks::runnable_info::RunnableInfo<D, ME>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let random_delay = random_range(0..150);

        std::thread::sleep(Duration::from_millis(random_delay));

        Box::new(())
    }
}

impl TaskResultable for DelayTask {
    type Result = ();

    fn downcast(_: TaskResult) -> Self::Result {}
}

fn main() -> Result<(), anyhow::Error> {
    let filter = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env()?;

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let mut server = DataloomServer::new(
        None,
        TracingStrategy {},
        SqliteStrategy::new_memory(),
        LocalMemory::new(),
    )?;

    let start = Instant::now();
    const WORK_AMOUNT: u64 = 1_500;

    for _ in 0..WORK_AMOUNT {
        server.get_task_handler().spawn_task(DelayTask {})?;
    }

    server.shutdown(true)?;
    let done = Instant::now();

    let total_time = done - start;
    println!("Took {:.2}s", total_time.as_secs_f32());

    println!(
        "Took {}ms per task",
        (total_time.as_millis() as f64 / WORK_AMOUNT as f64)
    );

    Ok(())
}
