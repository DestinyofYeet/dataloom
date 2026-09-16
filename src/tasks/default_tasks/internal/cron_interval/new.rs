use std::sync::mpsc::Sender;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        default_tasks::internal::cron_interval::{CronWorker, ReoccuringTaskList},
        taskhandler::TaskEvent,
    },
};

impl<D, ME> CronWorker<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub(crate) fn new(
        tasks: ReoccuringTaskList<D, ME>,
        mainloop_sender: Sender<TaskEvent<D, ME>>,
    ) -> Self {
        Self {
            tasks,
            sender: mainloop_sender,
        }
    }
}
