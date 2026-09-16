use crate::tasks::logstrategy::LogStrategyType;

#[derive(Clone)]
pub struct WorkerLogger {
    pub(crate) logger: LogStrategyType,
    pub(crate) worker_id: u64,
}
