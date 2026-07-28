use crate::tasks::{runnable_info::RunnableInfo, worker_logger::WorkerLogger};

impl RunnableInfo {
    pub fn get_logger(&self) -> &WorkerLogger {
        &self.logger
    }
}
