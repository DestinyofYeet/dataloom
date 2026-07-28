use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{
        task::{Task, TaskState},
        taskrunnable::TaskResultable,
    },
};

pub struct TaskRef<T, D>
where
    D: DatabaseStrategy,
{
    task: Arc<Mutex<Task<D>>>,
    _m: PhantomData<T>,
}

impl<T, D> TaskRef<T, D>
where
    T: TaskResultable,
    D: DatabaseStrategy,
{
    pub(crate) fn new(task: Arc<Mutex<Task<D>>>) -> Self {
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

impl<T, D> TaskRef<T, D>
where
    D: DatabaseStrategy,
{
    pub fn get_id(&self) -> Uuid {
        self.task.lock().expect("to get lock").get_id()
    }

    pub fn get_state(&self) -> TaskState {
        self.task.lock().expect("to get lock").get_state()
    }
}
