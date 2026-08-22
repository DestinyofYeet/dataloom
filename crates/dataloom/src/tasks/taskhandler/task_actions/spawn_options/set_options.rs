use crate::tasks::taskhandler::task_actions::spawn_options::{
    TaskSpawnOptionValue, TaskSpawnOptions,
};

impl TaskSpawnOptions {
    /// This task will not be ran by the global worker pool.
    /// A new worker will be created which will only run this task and then exit.
    pub fn set_long_running(mut self) -> Self {
        self.options.insert(TaskSpawnOptionValue::LongRunning);
        self
    }
}
