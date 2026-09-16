use std::sync::{Arc, Mutex, mpsc::Sender};

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{reoccurring_tasks::ReoccuringTask, taskhandler::TaskEvent},
};

pub(crate) type ReoccuringTaskList<D, ME> = Arc<Mutex<Vec<ReoccuringTask<D, ME>>>>;

pub(crate) struct CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(super) tasks: ReoccuringTaskList<D, ME>,
    pub(super) sender: Sender<TaskEvent<D, ME>>,
}
