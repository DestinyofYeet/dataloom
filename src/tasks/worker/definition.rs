use std::{
    cell::RefCell,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, Sender},
    },
    thread::{self, JoinHandle},
};

use tracing::trace;
use uuid::Uuid;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        task::{Task, TaskState},
        taskhandler::TaskEvent,
        worker::WorkerError,
    },
};

pub enum WorkerCommand<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    ProcessTask(Arc<Mutex<Task<D, ME>>>),
    Init,
    Close,
}

pub struct Worker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    id: u64,
    handle: RefCell<Option<JoinHandle<()>>>,
    to_thread: Sender<WorkerCommand<D, ME>>,
    current_task: Arc<Mutex<Option<Uuid>>>,
}

impl<D, ME> Worker<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub fn new(
        id: u64,
        to_handler: Sender<TaskEvent<D, ME>>,
        database_handle: Arc<D>,
        memory_handle: Arc<ME>,
    ) -> Result<Self, WorkerError> {
        let (tx, rx): (Sender<WorkerCommand<D, ME>>, Receiver<WorkerCommand<D, ME>>) =
            mpsc::channel();

        tx.send(WorkerCommand::Init)
            .map_err(|e| WorkerError::Channel(e.to_string()))?;

        let current_task = Arc::new(Mutex::new(None));

        let current_task1 = current_task.clone();

        let handle = thread::Builder::new()
            .name(format!("Worker {id}"))
            .spawn(move || {
                let worker_log = |text: &str| {
                    trace!("Worker {id}: {}", text);
                };

                while let Some(command) = rx.iter().next() {
                    match command {
                        WorkerCommand::ProcessTask(task) => {
                            let mut task = task.lock().expect("to get lock");

                            worker_log(&format!("Processing task {}", task.get_id()));
                            {
                                *current_task.lock().expect("to get lock") = Some(task.get_id());
                            }
                            task.set_state(TaskState::Running);
                            let result = task.run(
                                id,
                                to_handler.clone(),
                                database_handle.clone(),
                                memory_handle.clone(),
                            );
                            task.set_result(result);
                            task.set_state(TaskState::Done);
                            match to_handler.send(TaskEvent::TaskDone(task.get_id())) {
                                Ok(_) => {}
                                Err(e) => worker_log(&format!(
                                    "Failed to send message to TaskHandler: {e}"
                                )),
                            }
                            {
                                *current_task.lock().expect("to get lock") = None;
                            }
                        }

                        WorkerCommand::Init => worker_log("init"),
                        WorkerCommand::Close => break,
                    }
                }

                worker_log("exit")
            })
            .unwrap();

        Ok(Self {
            id,
            handle: RefCell::new(Some(handle)),
            to_thread: tx,
            current_task: current_task1,
        })
    }

    fn send_msg(&self, command: WorkerCommand<D, ME>) -> Result<(), WorkerError> {
        self.to_thread
            .send(command)
            .map_err(|e| WorkerError::Channel(e.to_string()))
    }

    pub fn schedule_task(&self, task: Arc<Mutex<Task<D, ME>>>) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::ProcessTask(task.clone()))
    }

    pub fn stop(&self) -> Result<(), WorkerError> {
        self.send_msg(WorkerCommand::Close)
    }

    pub fn get_task(&self) -> Option<Uuid> {
        *self.current_task.lock().expect("to get lock")
    }

    pub fn wait_for_join_handle(&self) -> Result<(), WorkerError> {
        let handle = self.handle.borrow_mut().take().unwrap();
        handle.join().map_err(|_| WorkerError::Join)
    }

    pub fn is_running(&self) -> bool {
        if let Some(handle) = self.handle.borrow().as_ref() {
            return !handle.is_finished();
        }

        false
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }
}
