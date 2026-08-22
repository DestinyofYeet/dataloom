use std::{collections::HashMap, rc::Rc};

use tracing::{error, warn};
use uuid::Uuid;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        taskhandler::{
            TaskEvent, TaskHandler,
            main_loop::{MainLoopData, WorkerList},
        },
        worker::Worker,
    },
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn respawn_dead_workers(
        data: &MainLoopData<D, ME>,
        workers: &mut WorkerList<D, ME>,
        task_worker_map: &mut HashMap<Uuid, Rc<Worker<D, ME>>>,
    ) {
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
                        _ = task_worker_map.remove(&uuid);
                        match data.sender.send(TaskEvent::TaskDone(uuid)) {
                            Ok(_) => {}
                            Err(e) => {
                                error!("Failed to send done message for crashed worker: {e}");
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
                    data.database.clone(),
                    data.memory.clone(),
                ) {
                    Ok(value) => {
                        warn!("Respawned worker {id}");
                        workers.push(Rc::new(value));
                    }
                    Err(e) => {
                        error!("Failed to respawn worker {id}: {e}");
                    }
                }
            }
        }
    }
}
