use chrono::{DateTime, Local, Utc};
use clap::Parser;
use django_rs::{
    django_rs_macro::{FromIter, SaveData},
    models::{
        MigrationKind, ModelMigration,
        column::{ColumnType, ColumnValue, CreateColumn, CreateOptions},
        search::SearchQuery,
        traits::model::Model,
    },
    server::{
        DjangoServer,
        database_strategy::{
            DatabaseStrategy, TransactionOptions, default_strategies::SqliteStrategy,
        },
        database_tasks::SaveModelTask,
    },
    tasks::{
        logstrategy::{LogStrategyType, default_strategies::tracing_strategy::TracingStrategy},
        runnable_info::RunnableInfo,
        taskrunnable::{TaskResultable, TaskRunnable},
        worker_logger::WorkerLogger,
    },
};
use serde::Serialize;
use std::{
    any::Any,
    sync::{Arc, LazyLock, Mutex},
    thread,
    time::Duration,
};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
pub struct Args {
    #[arg(short='v', long, action = clap::ArgAction::Count, help="Sets the verbose level. More v's more output")]
    verbose: u8,
}

#[derive(Default)]
pub struct PrintTask {}

impl PrintTask {
    pub fn new() -> Self {
        Self {}
    }
}

impl<D> TaskRunnable<D> for PrintTask
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Sync + Send> {
        let logger = info.get_logger();
        thread::sleep(Duration::from_millis(300));
        logger.info("print");

        Box::new(())
    }
}

impl TaskResultable for PrintTask {
    type Result = ();

    fn downcast(_result: django_rs::tasks::task::TaskResult) -> Self::Result {}
}

pub struct LongTask {
    pub stop: Arc<Mutex<bool>>,
}

impl<D> TaskRunnable<D> for LongTask
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Send + Sync> {
        let logger = info.get_logger();
        loop {
            logger.info("long task");
            thread::sleep(Duration::from_secs(2));

            {
                if *self.stop.lock().expect("to get lock") {
                    break;
                }
            }
        }

        match info.spawn_task(ShortTask {}) {
            Ok(_) => {}
            Err(e) => logger.error(&format!("Failed to spawn short task: {e}")),
        }

        Box::new(())
    }
}

impl TaskResultable for LongTask {
    type Result = ();

    fn downcast(_result: django_rs::tasks::task::TaskResult) -> Self::Result {}
}

pub struct ShortTask {}

impl<D> TaskRunnable<D> for ShortTask
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Send + Sync> {
        info.get_logger().info("short task");

        Box::new(())
    }
}

impl TaskResultable for ShortTask {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}

#[derive(Debug, FromIter, SaveData, Serialize)]
pub struct Group {
    id: Option<i64>,
    name: String,
}

impl Model for Group {
    const TABLE_NAME: &'static str = "groups";

    fn get_migration() -> &'static Vec<ModelMigration> {
        static MIGRATION: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "name",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable().set_unique(),
                    ),
                ]),
            )]
        });

        &MIGRATION
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }
}

#[derive(Debug, FromIter, SaveData)]
pub struct User {
    id: Option<i64>,
    username: String,
    email: String,
    created: DateTime<Utc>,
    group_id: i64,
}

impl Model for User {
    const TABLE_NAME: &'static str = "Users";

    fn get_migration() -> &'static Vec<ModelMigration> {
        static MIGRATIONS: LazyLock<Vec<ModelMigration>> = LazyLock::new(|| {
            vec![ModelMigration::new(
                0,
                MigrationKind::Create(vec![
                    CreateColumn::new(
                        "id",
                        ColumnType::Integer,
                        CreateOptions::default().set_primary_key(),
                    ),
                    CreateColumn::new(
                        "username",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "email",
                        ColumnType::String,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "created",
                        ColumnType::Date,
                        CreateOptions::default().set_non_nullable(),
                    ),
                    CreateColumn::new(
                        "group_id",
                        ColumnType::Integer,
                        CreateOptions::default()
                            // .set_non_nullable()
                            .set_foreign_key("groups", "id"),
                    ),
                ]),
            )]
        });

        &MIGRATIONS
    }

    fn get_id(&self) -> Option<i64> {
        self.id
    }

    fn set_id(&mut self, id: i64) {
        self.id = Some(id);
    }

    fn get_id_column_name(&self) -> &'static str {
        "id"
    }
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let level = match args.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    tracing_subscriber::fmt()
        .with_line_number(true)
        .with_env_filter(EnvFilter::new(level))
        .init();

    let mut server =
        DjangoServer::new(8, TracingStrategy {}, SqliteStrategy::new("tmp/db.sqlite"))?;

    let mut group = Group {
        id: None,
        name: "Test".to_string(),
    };

    server.get_database().migrate_model::<Group>()?;
    server.get_database().migrate_model::<User>()?;
    let db = server.get_database();
    let conn = db.get_connection();

    let stop = Arc::new(Mutex::new(false));

    println!("Spawning long task");
    server
        .get_task_handler()
        .spawn_task_long_running(LongTask { stop: stop.clone() })?;

    if let Some(found_group) = db.search_single_model::<Group>(
        &conn,
        SearchQuery::empty().add_constraint(("name", &group.name)),
    )? {
        group = found_group;
    } else {
        db.save_model(&conn, &mut group)?;
    };

    let mut user = User {
        id: None,
        username: "test".to_string(),
        email: "test@test.test".to_string(),
        created: Local::now().to_utc(),
        group_id: group.id.unwrap(),
    };

    db.save_model(&conn, &mut user)?;
    let conn = db.get_connection();
    let mut user = db
        .search_single_model::<User>(
            &conn,
            SearchQuery::empty().add_constraint(("id", ColumnValue::Integer(user.id.unwrap()))),
        )?
        .unwrap();

    user.username = "roflrofl".to_string();

    user.group_id = 5;

    let save_task = SaveModelTask::new(user);

    let task_handler = server.get_task_handler();
    task_handler.spawn_task::<PrintTask>(PrintTask::new())?;

    let task = task_handler.spawn_task(save_task);
    db.remove_model::<User>(
        &conn,
        &SearchQuery::empty().add_constraint(("username", "roflrofl")),
    )?;

    drop(conn);

    test(&server);

    {
        println!("Stopping long task");
        *stop.lock().expect("to get lock") = true;
    }
    std::thread::sleep(Duration::from_secs(5));
    server.shutdown()?;

    Ok(())
}

fn test<D>(server: &DjangoServer<D>)
where
    D: DatabaseStrategy + 'static,
{
    let db = server.get_database();
    db.with_transaction(|tx| {
        db.table_exists(&*tx, "hi").unwrap();
        db.manage_transaction(tx, TransactionOptions::Commit)
            .unwrap();
    })
    .unwrap();

    // let tx = db.get_transaction();

    // db.table_exists(&tx, "hi").unwrap();
    // db.manage_transaction(tx, TransactionOptions::Commit)
    // .unwrap();
}
