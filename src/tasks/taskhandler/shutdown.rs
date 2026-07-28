use crate::{
    server::database_strategy::DatabaseStrategy,
    tasks::taskhandler::{TaskEvent, TaskHandler, TaskHandlerError},
};

impl<D> TaskHandler<D>
where
    D: DatabaseStrategy,
{
    pub(crate) fn shutdown(&mut self) -> Result<(), TaskHandlerError> {
        if let Some(handle) = self.handle.take() {
            self.to_handler.send(TaskEvent::Shutdown)?;
            handle.join().map_err(|_| TaskHandlerError::Join)?;
        }
        Ok(())
    }
}
