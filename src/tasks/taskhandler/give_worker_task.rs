use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex},
};

use dataloom_db_core::traits::DatabaseStrategy;
use uuid::Uuid;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{task::Task, taskhandler::TaskHandler, worker::Worker},
};

impl<D, ME> TaskHandler<D, ME>
where
    D: DatabaseStrategy + 'static,
    ME: MemoryStrategy + 'static,
{
    pub(super) fn give_worker_task(
        task: Arc<Mutex<Task<D, ME>>>,
        worker: Rc<Worker<D, ME>>,
        task_worker_map: &mut HashMap<Uuid, Rc<Worker<D, ME>>>,
    ) {
        let task_id = task.lock().expect("to get lock").get_id();
        match worker.schedule_task(task) {
            Ok(_) => {
                task_worker_map.insert(task_id, worker.clone());
            }
            Err(e) => {
                eprintln!("Failed to schedule task: {e}")
            }
        }
    }
}
