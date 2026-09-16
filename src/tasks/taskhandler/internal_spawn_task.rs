use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

use dataloom_db_core::traits::DatabaseStrategy;
use tracing::{error, trace, warn};
use uuid::Uuid;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        taskhandler::{
            TaskHandler,
            main_loop::{HasTaskByWorkerId, MainLoopData, WorkerList, WrappedTask},
        },
        worker::Worker,
    },
};

pub(super) enum InternalTaskSpawnType<'a, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    Regular {
        task_queue: &'a mut VecDeque<WrappedTask<D, ME>>,
        workers: &'a mut WorkerList<D, ME>,
        task_worker_map: &'a mut HashMap<Uuid, Rc<Worker<D, ME>>>,
        worker_by_has_task: &'a mut HasTaskByWorkerId,
    },
    Long {
        long_worker_count: &'a mut u64,
    },
}

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn internal_spawn_task(
        task: WrappedTask<D, ME>,
        data: &MainLoopData<D, ME>,
        accept_new_tasks: bool,

        task_type: InternalTaskSpawnType<D, ME>,
    ) {
        if !accept_new_tasks {
            return;
        }

        match task_type {
            InternalTaskSpawnType::Regular {
                task_queue,
                workers,
                task_worker_map,
                worker_by_has_task,
            } => {
                Self::respawn_dead_workers(data, workers, task_worker_map);

                for worker in workers.iter() {
                    if !worker_by_has_task
                        .get(&worker.get_id())
                        .copied()
                        .unwrap_or(false)
                    {
                        Self::give_worker_task(
                            task,
                            worker.clone(),
                            task_worker_map,
                            worker_by_has_task,
                        );
                        return;
                    }
                }

                task_queue.push_back(task);
                trace!("queue size: {}", task_queue.len())
            }

            InternalTaskSpawnType::Long { long_worker_count } => {
                *long_worker_count += 1;

                let worker = match Worker::new(
                    *long_worker_count + data.max_workers,
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
}
