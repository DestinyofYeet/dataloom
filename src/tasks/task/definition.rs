use std::{
    any::Any,
    sync::{Arc, mpsc::Sender},
};

use uuid::Uuid;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        logstrategy::LogStrategyType, runnable_info::RunnableInfo, taskhandler::TaskEvent,
        taskrunnable::TaskRunnable, worker_logger::WorkerLogger,
    },
};

pub type Runnable<D, ME> = Box<dyn TaskRunnable<D, ME> + Sync + Send>;

pub type TaskResult = Box<dyn Any + Send + Sync>;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum TaskState {
    Queued,
    Running,
    Done,
}

pub(crate) struct Task<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    id: Uuid,
    runnable: Runnable<D, ME>,
    logger: LogStrategyType,
    state: TaskState,
    result: Option<TaskResult>,
}

impl<D, ME> Task<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(runnable: Runnable<D, ME>, logger: LogStrategyType) -> Self {
        Self {
            id: Uuid::new_v4(),
            runnable,
            logger,
            state: TaskState::Queued,
            result: None,
        }
    }

    pub(crate) fn run(
        &mut self,
        worker_id: u64,
        to_handler: Sender<TaskEvent<D, ME>>,
        database_handle: Arc<D>,
        memory_handle: Arc<ME>,
    ) -> TaskResult {
        let logger = WorkerLogger::new(self.logger.clone(), worker_id);
        let info = RunnableInfo::new(logger, to_handler, database_handle, memory_handle);
        self.runnable.run(info)
    }

    pub(crate) fn set_result(&mut self, result: TaskResult) {
        self.result = Some(result);
    }

    pub(crate) fn get_result(&mut self) -> Option<TaskResult> {
        self.result.take()
    }

    #[inline(always)]
    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn set_state(&mut self, state: TaskState) {
        self.state = state;
    }

    pub fn get_state(&self) -> TaskState {
        self.state
    }
}
