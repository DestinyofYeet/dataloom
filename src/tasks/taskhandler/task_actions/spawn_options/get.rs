use crate::tasks::taskhandler::task_actions::spawn_options::{
    TaskSpawnOptionValue, TaskSpawnOptions,
};

impl TaskSpawnOptions {
    pub fn is_long_running(&self) -> bool {
        self.options.contains(&TaskSpawnOptionValue::LongRunning)
    }
}
