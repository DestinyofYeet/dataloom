use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use dataloom_db_core::traits::DatabaseStrategy;
use uuid::Uuid;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        task::{Task, TaskState},
        taskrunnable::TaskResultable,
    },
};

pub struct TaskRef<T, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    task: Arc<Mutex<Task<D, ME>>>,
    _m: PhantomData<T>,
}

impl<T, D, ME> TaskRef<T, D, ME>
where
    T: TaskResultable,
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(task: Arc<Mutex<Task<D, ME>>>) -> Self {
        Self {
            task,
            _m: PhantomData,
        }
    }

    pub fn get_result(&self) -> Option<T::Result> {
        let result = self.task.lock().expect("to get lock").get_result();
        Some(T::downcast(result?))
    }
}

impl<T, D, ME> TaskRef<T, D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn get_id(&self) -> Uuid {
        self.task.lock().expect("to get lock").get_id()
    }

    pub fn get_state(&self) -> TaskState {
        self.task.lock().expect("to get lock").get_state()
    }
}
