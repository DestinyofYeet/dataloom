use std::{any::Any, marker::PhantomData};

use crate::{
    core::{search::SearchQuery, traits::model::Model},
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{runnable_info::RunnableInfo, taskrunnable::TaskRunnable},
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

impl<D, MO, ME> TaskRunnable<D, ME> for RemoveModelTask<MO>
where
    D: DatabaseStrategy,
    MO: Model,
    ME: MemoryStrategy,
{
    fn run(&mut self, info: RunnableInfo<D, ME>) -> Box<dyn Any + Send + Sync> {
        let db = info.get_database();
        let conn = db.get_connection();
        Box::new(db.remove_model::<MO>(&conn, self.search.clone()))
    }
}
