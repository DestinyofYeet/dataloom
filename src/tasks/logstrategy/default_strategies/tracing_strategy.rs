use std::panic::Location;

use tracing::{debug, error, info, trace, warn};

use crate::tasks::logstrategy::LogStrategy;

pub struct TracingStrategy {}

impl LogStrategy for TracingStrategy {
    fn warn(&self, worker_id: u64, message: &str, location: &'static Location<'static>) {
        let file = location.file();
        let line = location.line();
        warn!(
            target: "dataloom::worker",
            "[Worker {worker_id}] ({file}:{line}) {message}"
        );
    }

    fn error(&self, worker_id: u64, message: &str, location: &'static Location<'static>) {
        let file = location.file();
        let line = location.line();
        error!(
            target: "dataloom::worker",
            "[Worker {worker_id}] ({file}:{line}) {message}"
        );
    }

    fn info(&self, worker_id: u64, message: &str, location: &'static Location<'static>) {
        let file = location.file();
        let line = location.line();
        info!(
            target: "dataloom::worker",
            "[Worker {worker_id}] ({file}:{line}) {message}"
        );
    }

    fn debug(&self, worker_id: u64, message: &str, location: &'static Location<'static>) {
        let file = location.file();
        let line = location.line();
        debug!(
            target: "dataloom::worker",
            "[Worker {worker_id}] ({file}:{line}) {message}"
        );
    }

    fn trace(&self, worker_id: u64, message: &str, location: &'static Location<'static>) {
        let file = location.file();
        let line = location.line();
        trace!(
            target: "dataloom::worker",
            "[Worker {worker_id}] ({file}:{line}) {message}"
        );
    }
}
