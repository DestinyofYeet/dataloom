use std::time::Duration;

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
    pub(crate) fn shutdown(&mut self, wait: bool) -> Result<(), TaskHandlerError> {
        if let Some(handle) = self.handle.take() {
            self.to_handler.send(TaskEvent::NoNewTask)?;
            if wait {
                loop {
                    let (sender, reciever) = oneshot::channel();

                    self.to_handler
                        .send(TaskEvent::GetQueueSize { response: sender })?;

                    if reciever
                        .recv()
                        .map_err(|e| TaskHandlerError::Recieve(e.to_string()))?
                        == 0
                    {
                        break;
                    }

                    std::thread::sleep(Duration::from_millis(200));
                }
            }
            self.to_handler.send(TaskEvent::Shutdown)?;
            handle.join().map_err(|_| TaskHandlerError::Join)?;
        }
        Ok(())
    }
}
