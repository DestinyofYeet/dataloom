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
        taskhandler::{TaskEvent, TaskHandler, TaskSubscriberEvent, task_actions::TaskActions},
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

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn main_loop(data: MainLoopData<D, ME>) {
        info!("Number of workers: {}", data.max_workers);
        let mut workers: WorkerList<D, ME> = Vec::with_capacity(data.max_workers as usize);
        let mut task_queue: VecDeque<Arc<Mutex<Task<D, ME>>>> = VecDeque::new();
        let mut task_worker_map: HashMap<Uuid, Rc<Worker<D, ME>>> = HashMap::new();

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

        'mainloop: while let Some(command) = data.recv.iter().next() {
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
                    Self::respawn_dead_workers(&data, &mut workers, &mut task_worker_map);

                    for worker in workers.iter() {
                        if worker.get_task().is_none() {
                            Self::give_worker_task(task, worker.clone(), &mut task_worker_map);

                            continue 'mainloop;
                        }
                    }

                    task_queue.push_back(task);
                    trace!("queue size: {}", task_queue.len());
                }

                TaskEvent::TaskDone(uuid) => {
                    if let Some(sender) = subscribers.get(&uuid) {
                        match sender.send(TaskSubscriberEvent::TaskDone) {
                            Ok(_) => {}
                            Err(e) => warn!("Failed to send message to subscriber: {e}"),
                        }
                    }

                    if let Some(worker) = task_worker_map.remove(&uuid)
                        && let Some(task) = task_queue.pop_front()
                    {
                        Self::give_worker_task(task, worker.clone(), &mut task_worker_map);
                        trace!("queue size: {}", task_queue.len());
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
                    long_worker_count += 1;

                    let worker = match Worker::new(
                        long_worker_count + data.max_workers,
                        data.sender.clone(),
                        data.task_actions.clone(),
                        data.database.clone(),
                        data.memory.clone(),
                    ) {
                        Ok(value) => value,
                        Err(e) => {
                            warn!("Failed to spawn long running worker {long_worker_count}: {e}");
                            return;
                        }
                    };

                    match worker.schedule_task(task) {
                        Ok(_) => {}
                        Err(e) => error!("Failed to schedule long running task: {e}"),
                    }

                    // immediately send the stop command. It won't get processed until the task has finished
                    match worker.stop() {
                        Ok(_) => {}
                        Err(e) => error!("Failed to stop long running worker: {e}"),
                    };
                }
            }
        }

        debug!("TaskHandler exited")
    }
}
