use std::{any::Any, sync::Arc};

use crate::{
    models::traits::{
        from_iter::FromIter,
        model::Model,
        save_data::{SaveData, ValidateSaveData},
    },
    server::{
        database_strategy::{DatabaseStrategy, DatabaseStrategyError},
        memory_strategy::MemoryStrategy,
    },
    tasks::{
        runnable_info::RunnableInfo,
        taskrunnable::{TaskResultable, TaskRunnable},
        worker_logger::WorkerLogger,
    },
};

pub struct SaveModelTask<M>
where
    M: Model,
{
    model: M,
}

impl<M> SaveModelTask<M>
where
    M: Model,
{
    pub fn new(model: M) -> Self {
        Self { model }
    }

    pub fn get_model(&self) -> &M {
        &self.model
    }
}

impl<D, MO, ME> TaskRunnable<D, ME> for SaveModelTask<MO>
where
    D: DatabaseStrategy,
    MO: Model + SaveData + FromIter + ValidateSaveData + Send + Sync,
    ME: MemoryStrategy,
{
    fn run(&mut self, info: RunnableInfo<D, ME>) -> Box<dyn Any + Send + Sync> {
        let logger = info.get_logger();
        let db = info.get_database();
        let conn = db.get_connection();
        match db.save_model(&conn, &mut self.model) {
            Ok(_) => {}
            Err(e) => logger.error(&format!("Failed to save model: {e}")),
        };
        Box::new(self.model.get_id())
    }
}

impl<M> TaskResultable for SaveModelTask<M>
where
    M: Model + SaveData + FromIter + ValidateSaveData,
{
    type Result = Option<i64>;

    fn downcast(result: crate::tasks::task::TaskResult) -> Self::Result {
        *result.downcast().expect("to parse result")
    }
}
