use std::sync::mpsc::Sender;

use crate::tasks::{taskhandler::TaskEvent, worker_logger::WorkerLogger};

pub struct RunnableInfo {
    pub(super) logger: WorkerLogger,
    pub(super) to_handler: Sender<TaskEvent>,
}
