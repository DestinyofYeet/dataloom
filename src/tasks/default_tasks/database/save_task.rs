use std::any::Any;

use dataloom_db_core::traits::{
    DatabaseStrategy, DatabaseStrategyError,
    from_iter::FromIter,
    model::Model,
    save_data::{SaveData, ValidateSaveData},
};

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        runnable_info::RunnableInfo,
        taskrunnable::{TaskResultable, TaskRunnable},
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
        let db = info.get_database();
        let conn = db.get_connection();
        match db.save_model(&conn, &mut self.model) {
            Ok(_) => {}
            Err(e) => {
                return Box::new(Err::<i64, DatabaseStrategyError>(e));
            }
        };

        Box::new(Ok::<i64, DatabaseStrategyError>(
            self.model.get_id().unwrap(),
        ))
    }
}

impl<M> TaskResultable for SaveModelTask<M>
where
    M: Model + SaveData + FromIter + ValidateSaveData,
{
    type Result = Result<i64, DatabaseStrategyError>;

    fn downcast(result: crate::tasks::task::TaskResult) -> Self::Result {
        *result.downcast().expect("to parse result")
    }
}
