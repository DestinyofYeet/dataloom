use crate::tasks::taskhandler::{TaskEvent, TaskHandler, TaskHandlerError};

impl TaskHandler {
    pub(crate) fn shutdown(&mut self) -> Result<(), TaskHandlerError> {
        if let Some(handle) = self.handle.take() {
            self.to_handler.send(TaskEvent::Shutdown)?;
            handle.join().map_err(|_| TaskHandlerError::Join)?;
        }
        Ok(())
    }
}
