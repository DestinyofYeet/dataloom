use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub(super) enum TaskSpawnOptionValue {
    LongRunning,
}

pub struct TaskSpawnOptions {
    pub(super) options: HashSet<TaskSpawnOptionValue>,
}
