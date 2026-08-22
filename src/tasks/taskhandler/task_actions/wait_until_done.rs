use std::sync::mpsc::channel;

use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::{
        task::TaskState,
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

        if task.get_state() == TaskState::Done {
            return Ok(());
        }

        self.to_task_handler.send(TaskEvent::RegisterSubscriber {
            for_task: task.get_id(),
            subscriber: tx,
        })?;

        // There can be the following issue:
        // - First check if task is done
        // - Worker sends Done notification
        // - Register the listener
        // - :( we don't get the signal
        //
        // We check a second time here, just in case the above happened
        // There should be a proper fix at some point, because if this happens too often,
        // it could exhaust the memory of the system, since the uuid and channel tx are stored in a hashmap.
        // But for now it should be a good enough fix to reliably call wait_until_done and be sure we don't get stuck
        if task.get_state() == TaskState::Done {
            return Ok(());
        }

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
