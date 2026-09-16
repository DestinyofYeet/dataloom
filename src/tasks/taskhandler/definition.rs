use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread::{self, JoinHandle},
};

use dataloom_db_core::traits::DatabaseStrategy;
use uuid::Uuid;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::internal::cron_interval::{CronWorker, ReoccuringTaskList},
        logstrategy::LogStrategyType,
        task::Task,
        taskhandler::{main_loop::MainLoopData, task_actions::TaskActions},
    },
};

pub(crate) enum TaskEvent<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    Shutdown,
    ProcessTask(Arc<Mutex<Task<D, ME>>>),
    ProcessLongTask(Arc<Mutex<Task<D, ME>>>),
    TaskDone(Uuid),
    RegisterSubscriber {
        for_task: Uuid,
        subscriber: Sender<TaskSubscriberEvent>,
    },

    UnregisterSubscriber {
        for_task: Uuid,
    },

    GetQueueSize {
        response: oneshot::Sender<u64>,
    },

    NoNewTask,
}

pub enum TaskSubscriberEvent {
    CommInit,
    TaskDone,
}

#[allow(dead_code)]
pub struct TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) log_strategy: LogStrategyType,
    pub(super) max_workers: u64,

    pub(super) to_handler: Sender<TaskEvent<D, ME>>,

    pub(super) task_actions: Arc<TaskActions<D, ME>>,

    pub(super) handle: Option<JoinHandle<()>>,
    pub(super) database_handle: Arc<D>,
    pub(super) memory_handle: Arc<ME>,

    pub(super) reoccuring_tasks: ReoccuringTaskList<D, ME>,
}

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub fn new(
        max_workers: u64,
        log_strategy: LogStrategyType,
        database_handle: Arc<D>,
        memory_handle: Arc<ME>,
    ) -> Self {
        let (sender, receiver) = mpsc::channel();

        let task_actions = Arc::new(TaskActions::new(sender.clone(), log_strategy.clone()));

        let data = MainLoopData {
            recv: receiver,
            sender: sender.clone(),
            max_workers,
            database: database_handle.clone(),
            memory: memory_handle.clone(),
            task_actions: task_actions.clone(),
            log_strategy: log_strategy.clone(),
        };

        let handle = thread::Builder::new()
            .name("TaskHandler".to_string())
            .spawn(move || {
                TaskHandler::main_loop(data);
            })
            .unwrap();

        let reoccuring_task_list = Arc::new(Mutex::new(Vec::new()));

        let cron_worker = CronWorker::new(reoccuring_task_list.clone(), sender.clone());

        let myself = Self {
            max_workers,
            log_strategy,
            to_handler: sender,
            handle: Some(handle),
            database_handle,
            memory_handle,
            task_actions,
            reoccuring_tasks: reoccuring_task_list,
        };

        myself
            .spawn_task_long_running(cron_worker)
            .expect("to be able to spawn cron_worker");

        myself
    }
}
