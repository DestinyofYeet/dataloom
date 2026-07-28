use std::sync::mpsc::Sender;

use crate::tasks::{
    runnable_info::RunnableInfo, taskhandler::TaskEvent, worker_logger::WorkerLogger,
};

impl RunnableInfo {
    pub fn new(logger: WorkerLogger, to_handler: Sender<TaskEvent>) -> Self {
        Self { logger, to_handler }
    }
}
