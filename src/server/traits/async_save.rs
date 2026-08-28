use dataloom_db_core::traits::{
    DatabaseStrategy, from_iter::FromIter, model::Model, save_data::SaveData,
};

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::database::SaveModelTask,
        taskhandler::{TaskHandler, TaskHandlerError},
        taskref::TaskRef,
    },
};

pub trait AsyncSave<T, D, ME>
where
    Self: Sized,
    T: Model + FromIter + SaveData,
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn save_async(
        self,
        task_handler: &TaskHandler<D, ME>,
    ) -> Result<TaskRef<SaveModelTask<T>, D, ME>, TaskHandlerError>;
}

impl<T, D, ME> AsyncSave<T, D, ME> for T
where
    T: Model + FromIter + SaveData + Send + Sync + 'static,
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    fn save_async(
        self,
        task_handler: &TaskHandler<D, ME>,
    ) -> Result<TaskRef<SaveModelTask<T>, D, ME>, TaskHandlerError> {
        let task = SaveModelTask::new(self);
        task_handler.spawn_task(task)
    }
}
