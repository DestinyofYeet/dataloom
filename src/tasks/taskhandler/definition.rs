use std::{
    sync::{
        Arc, Mutex,
        mpsc::{self, Sender},
    },
    thread::{self, JoinHandle},
};

use uuid::Uuid;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{logstrategy::LogStrategyType, task::Task, taskhandler::main_loop::MainLoopData},
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
}

pub enum TaskSubscriberEvent {
    CommInit,
    TaskDone,
}

pub struct TaskHandler<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) log_strategy: LogStrategyType,
    pub(super) max_workers: u64,

    pub(super) to_handler: Sender<TaskEvent<D, ME>>,

    pub(super) handle: Option<JoinHandle<()>>,
    pub(super) database_handle: Arc<D>,
    pub(super) memory_handle: Arc<ME>,
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

        let data = MainLoopData {
            recv: receiver,
            sender: sender.clone(),
            max_workers,
            database: database_handle.clone(),
            memory: memory_handle.clone(),
        };

        let handle = thread::Builder::new()
            .name("TaskHandler".to_string())
            .spawn(move || {
                TaskHandler::main_loop(data);
            })
            .unwrap();

        Self {
            max_workers,
            log_strategy,
            to_handler: sender,
            handle: Some(handle),
            database_handle,
            memory_handle,
        }
    }
}
