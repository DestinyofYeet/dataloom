use std::collections::HashSet;

use crate::tasks::taskhandler::task_actions::spawn_options::TaskSpawnOptions;

impl TaskSpawnOptions {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }
}
