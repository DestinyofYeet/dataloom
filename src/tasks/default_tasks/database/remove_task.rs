use std::{any::Any, marker::PhantomData, sync::Arc};

use crate::{
    models::{search::SearchQuery, traits::model::Model},
    server::database_strategy::DatabaseStrategy,
    tasks::{runnable_info::RunnableInfo, taskrunnable::TaskRunnable, worker_logger::WorkerLogger},
};

pub struct RemoveModelTask<M>
where
    M: Model,
{
    search: SearchQuery,
    marker: PhantomData<M>,
}

impl<M> RemoveModelTask<M>
where
    M: Model,
{
    pub fn new(search: SearchQuery) -> Box<Self> {
        Box::new(Self {
            search,
            marker: PhantomData,
        })
    }
}

impl<D, M> TaskRunnable<D> for RemoveModelTask<M>
where
    D: DatabaseStrategy,
    M: Model,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn Any + Send + Sync> {
        let db = info.get_database();
        let conn = db.get_connection();
        Box::new(db.remove_model::<M>(&conn, &self.search))
    }
}
