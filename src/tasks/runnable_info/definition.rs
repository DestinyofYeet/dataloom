use std::sync::{Arc, mpsc::Sender};

use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::{taskhandler::TaskEvent, worker_logger::WorkerLogger},
};

pub struct RunnableInfo<D>
where
    D: DatabaseStrategy,
{
    pub(super) logger: WorkerLogger,
    pub(super) to_handler: Sender<TaskEvent<D>>,
    pub(super) database_handle: Arc<D>,
}
