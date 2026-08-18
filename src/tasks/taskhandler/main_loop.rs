use std::{
    collections::HashMap,
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};

use tracing::{debug, error, warn};
use uuid::Uuid;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        taskhandler::{TaskEvent, TaskHandler, TaskSubscriberEvent, task_actions::TaskActions},
        worker::Worker,
    },
};

pub(super) struct MainLoopData<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) recv: Receiver<TaskEvent<D, ME>>,
    pub(super) sender: Sender<TaskEvent<D, ME>>,
    pub(super) max_workers: u64,
    pub(super) database: &'static D,
    pub(super) memory: &'static ME,
    pub(super) task_actions: Arc<TaskActions<D, ME>>,
}

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn main_loop(data: MainLoopData<D, ME>) {
        let mut workers: Vec<Worker<D, ME>> = Vec::with_capacity(data.max_workers as usize);

        for i in 0..data.max_workers {
            workers.push(
                Worker::new(
                    i,
                    data.sender.clone(),
                    data.task_actions.clone(),
                    data.database,
                    data.memory,
                )
                .expect("to) create workers"),
            );
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
                    let active_workers: u64 = workers
                        .iter()
                        .map(|e| if e.is_running() { 1 } else { 0 })
                        .sum();

                    if active_workers != data.max_workers {
                        let diff = data.max_workers - active_workers;

                        let mut respawn_worker_ids = Vec::with_capacity(diff as usize);

                        for worker in workers.iter() {
                            if !worker.is_running() {
                                let id = worker.get_id();
                                respawn_worker_ids.push(id);
                                warn!("Worker {id} is not running! It probably crashed.");
                                if let Some(uuid) = worker.get_task() {
                                    warn!("Worker {id} had task id {uuid}.");
                                    match data.sender.send(TaskEvent::TaskDone(uuid)) {
                                        Ok(_) => {}
                                        Err(e) => {
                                            error!(
                                                "Failed to send done message for crashed worker: {e}"
                                            );
                                        }
                                    }
                                }
                            }
                        }

                        workers.retain(|e| e.is_running());

                        for id in respawn_worker_ids {
                            match Worker::new(
                                id,
                                data.sender.clone(),
                                data.task_actions.clone(),
                                data.database,
                                data.memory,
                            ) {
                                Ok(value) => {
                                    warn!("Respawned worker {id}");
                                    workers.push(value);
                                }
                                Err(e) => {
                                    error!("Failed to respawn worker {id}: {e}");
                                }
                            }
                        }
                    }

                    let random_worker = rand::random_range(0..workers.len());
                    let worker = &workers[random_worker as usize];

                    match worker.schedule_task(task) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Failed to schedule task: {e}");
                        }
                    }
                }
                TaskEvent::TaskDone(uuid) => {
                    if let Some(sender) = subscribers.get(&uuid) {
                        match sender.send(TaskSubscriberEvent::TaskDone) {
                            Ok(_) => {}
                            Err(e) => warn!("Failed to send message to subscriber: {e}"),
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
                    long_worker_count += 1;

                    let worker = match Worker::new(
                        long_worker_count + data.max_workers,
                        data.sender.clone(),
                        data.task_actions.clone(),
                        data.database,
                        data.memory,
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
