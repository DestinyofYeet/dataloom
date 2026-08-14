use std::sync::mpsc::channel;

use crate::{
    server::{database_strategy::DatabaseStrategy, memory_strategy::MemoryStrategy},
    tasks::{
        taskhandler::{
            TaskEvent, TaskHandlerError, TaskSubscriberEvent, task_actions::TaskActions,
        },
        taskref::TaskRef,
    },
};

impl<D, ME> TaskActions<D, ME>
where
    D: DatabaseStrategy,
    ME: MemoryStrategy,
{
    pub fn wait_until_done<T>(&self, task: &TaskRef<T, D, ME>) -> Result<(), TaskHandlerError> {
        let (tx, rx) = channel();

        self.to_task_handler.send(TaskEvent::RegisterSubscriber {
            for_task: task.get_id(),
            subscriber: tx,
        })?;

        while let Some(message) = rx.iter().next() {
            match message {
                TaskSubscriberEvent::CommInit => {}
                TaskSubscriberEvent::TaskDone => {
                    self.to_task_handler.send(TaskEvent::UnregisterSubscriber {
                        for_task: task.get_id(),
                    })?;
                    break;
                }
            }
        }

        Ok(())
    }
}
