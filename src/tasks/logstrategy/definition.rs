use std::{panic::Location, sync::Arc};

pub type LogStrategyType = Arc<dyn LogStrategy + Send + Sync>;

pub trait LogStrategy {
    fn warn(&self, worker_id: u64, message: &str, location: &'static Location<'static>);
    fn error(&self, worker_id: u64, message: &str, location: &'static Location<'static>);
    fn info(&self, worker_id: u64, message: &str, location: &'static Location<'static>);
    fn debug(&self, worker_id: u64, message: &str, location: &'static Location<'static>);
    fn trace(&self, worker_id: u64, message: &str, location: &'static Location<'static>);
}
