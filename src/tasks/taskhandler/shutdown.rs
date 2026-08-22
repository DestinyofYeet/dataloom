use dataloom_db_core::traits::DatabaseStrategy;

use crate::{
    server::memory_strategy::MemoryStrategy,
    tasks::taskhandler::{TaskEvent, TaskHandler, TaskHandlerError},
};

impl<D, M> TaskHandler<D, M>
where
    D: DatabaseStrategy,
    M: MemoryStrategy,
{
    pub(crate) fn shutdown(&mut self) -> Result<(), TaskHandlerError> {
        if let Some(handle) = self.handle.take() {
            self.to_handler.send(TaskEvent::Shutdown)?;
            handle.join().map_err(|_| TaskHandlerError::Join)?;
        }
        Ok(())
    }
}
