use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    models::{
        search::SearchQuery,
        traits::{from_iter::FromIter, model::Model},
    },
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError},
    tasks::{
        runnable_info::RunnableInfo,
        taskrunnable::{TaskResultable, TaskRunnable},
        worker_logger::WorkerLogger,
    },
};

pub struct GetModelTask<'a, M>
where
    M: Model,
{
    search: SearchQuery,
    _m: PhantomData<&'a M>,
}

impl<'a, M> GetModelTask<'a, M>
where
    M: Model,
{
    pub fn new(search: SearchQuery) -> Self {
        Self {
            search,
            _m: PhantomData,
        }
    }
}

impl<'a, D, M> TaskRunnable<D> for GetModelTask<'a, M>
where
    D: DatabaseStrategy,
    M: Model + FromIter + Send + Sync + 'static,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Send + Sync> {
        let db = info.get_database();
        let result = db.search_single_model::<M>(&db.get_connection(), self.search.clone());

        Box::new(result)
    }
}

impl<'a, M> TaskResultable for GetModelTask<'a, M>
where
    M: Model + FromIter + 'static,
{
    type Result = Result<Option<M>, DatabaseStrategyError>;

    fn downcast(result: crate::tasks::task::TaskResult) -> Self::Result {
        *result.downcast().expect("to downcast")
    }
}
