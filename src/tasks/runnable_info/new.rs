use std::sync::{Arc, mpsc::Sender};

use crate::{
    server::database_strategy::{self, DatabaseStrategy},
    tasks::{runnable_info::RunnableInfo, taskhandler::TaskEvent, worker_logger::WorkerLogger},
};

impl<D> RunnableInfo<D>
where
    D: DatabaseStrategy,
{
    pub fn new(
        logger: WorkerLogger,
        to_handler: Sender<TaskEvent<D>>,
        database_handle: Arc<D>,
    ) -> Self {
        Self {
            logger,
            to_handler,
            database_handle,
        }
    }
}
