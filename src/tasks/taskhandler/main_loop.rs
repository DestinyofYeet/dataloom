use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
    sync::{
        Arc, Mutex,
        mpsc::{Receiver, Sender},
    },
};

use dataloom_db_core::traits::DatabaseStrategy;
use tracing::{debug, error, info, trace, warn};
use uuid::Uuid;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        task::Task,
        taskhandler::{
            TaskEvent, TaskHandler, TaskSubscriberEvent,
            internal_spawn_task::InternalTaskSpawnType, task_actions::TaskActions,
        },
        worker::Worker,
    },
};

pub(super) struct MainLoopData<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) recv: Receiver<TaskEvent<D, ME>>,
    pub(super) sender: Sender<TaskEvent<D, ME>>,
    pub(super) max_workers: u64,
    pub(super) database: Arc<D>,
    pub(super) memory: Arc<ME>,
    pub(super) task_actions: Arc<TaskActions<D, ME>>,
}

pub(super) type WorkerList<D, ME> = Vec<Rc<Worker<D, ME>>>;
pub(super) type WrappedTask<D, ME> = Arc<Mutex<Task<D, ME>>>;
pub(super) type WorkerByHasTask = HashMap<u64, bool>;

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn main_loop(data: MainLoopData<D, ME>) {
        info!("Number of workers: {}", data.max_workers);

        let mut workers: WorkerList<D, ME> = Vec::with_capacity(data.max_workers as usize);
        let mut task_queue: VecDeque<WrappedTask<D, ME>> = VecDeque::new();
        let mut task_worker_map: HashMap<Uuid, Rc<Worker<D, ME>>> = HashMap::new();
        let mut worker_by_has_task: WorkerByHasTask = HashMap::new();

        let mut accept_new_tasks = true;

        for i in 0..data.max_workers {
            workers.push(Rc::new(
                Worker::new(
                    i,
                    data.sender.clone(),
                    data.task_actions.clone(),
                    data.database.clone(),
                    data.memory.clone(),
                )
                .expect("to create workers"),
            ));
        }

        let mut subscribers = HashMap::<Uuid, Sender<TaskSubscriberEvent>>::new();

        let mut long_worker_count: u64 = 0;

        while let Some(command) = data.recv.iter().next() {
            match command {
                TaskEvent::Shutdown => {
                    for worker in workers.iter() {
                        match worker.stop() {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Could not send stop to worker {}: {e}", worker.get_id());
                            }
                        }
                    }

                    for worker in workers {
                        match worker.wait_for_join_handle() {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to wait for worker {}: {e}", worker.get_id());
                            }
                        }
                    }
                    break;
                }
                TaskEvent::ProcessTask(task) => {
                    Self::internal_spawn_task(
                        task,
                        &data,
                        accept_new_tasks,
                        InternalTaskSpawnType::Regular {
                            workers: &mut workers,
                            task_worker_map: &mut task_worker_map,
                            task_queue: &mut task_queue,
                            worker_by_has_task: &mut worker_by_has_task,
                        },
                    );
                }

                TaskEvent::TaskDone(uuid) => {
                    if let Some(sender) = subscribers.get(&uuid) {
                        match sender.send(TaskSubscriberEvent::TaskDone) {
                            Ok(_) => {}
                            Err(e) => warn!("Failed to send message to subscriber: {e}"),
                        }
                    }

                    if let Some(worker) = task_worker_map.remove(&uuid) {
                        worker_by_has_task.insert(worker.get_id(), false);

                        if let Some(task) = task_queue.pop_front() {
                            Self::give_worker_task(
                                task,
                                worker.clone(),
                                &mut task_worker_map,
                                &mut worker_by_has_task,
                            );
                            trace!("queue size: {}", task_queue.len());
                        }
                    }
                }
                TaskEvent::RegisterSubscriber {
                    for_task,
                    subscriber,
                } => {
                    subscribers.insert(for_task, subscriber);
                }

                TaskEvent::UnregisterSubscriber { for_task } => {
                    if let Some(sender) = subscribers.remove(&for_task) {
                        drop(sender)
                    }
                }
                TaskEvent::ProcessLongTask(task) => {
                    Self::internal_spawn_task(
                        task,
                        &data,
                        accept_new_tasks,
                        InternalTaskSpawnType::Long {
                            long_worker_count: &mut long_worker_count,
                        },
                    );
                }

                TaskEvent::GetQueueSize { response } => {
                    match response.send(task_queue.len() as u64) {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Failed to respond to GetQueueSize request: {e}")
                        }
                    }
                }

                TaskEvent::NoNewTask => {
                    accept_new_tasks = false;
                }
            }
        }

        debug!("TaskHandler exited")
    }
}
