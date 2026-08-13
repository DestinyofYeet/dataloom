use crate::models::column::{ModifyColumn, create::CreateColumn};

pub enum MigrationKind {
    Create(Vec<CreateColumn>),
    Modify(Vec<ModifyColumn>),
}

pub struct ModelMigration {
    pub(crate) ordering: u64,
    pub(crate) kind: MigrationKind,
}

impl ModelMigration {
    pub fn new(ordering: u64, kind: MigrationKind) -> Self {
        Self { ordering, kind }
    }
}
