use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    MigrationKind, ModelMigration,
    column::{ColumnType, ModifyColumnOptionsValues},
};

pub trait Model {
    const TABLE_NAME: &'static str;

    /// This function should return the migration path for this Model
    fn get_migration() -> &'static Vec<ModelMigration>;

    /// This function controls wether the model is saved or inserted into the database
    fn get_id(&self) -> Option<i64>;

    /// This function returns the name of the id field
    fn get_id_column_name(&self) -> &'static str;

    /// This function sets the id returned by the database
    fn set_id(&mut self, id: i64);

    /// This function returns the latest name of a column by traversing the migration path.
    /// An option of None indicates that the Column was dropped in the migration path
    fn get_latest_column_name(initial_name: &str) -> Option<String> {
        let mut past_names = vec![initial_name.to_string()];
        let mut name = Some(String::from(initial_name));

        for migration in Self::get_migration()
            .iter()
            .sorted_by_key(|item| item.ordering)
        {
            match &migration.kind {
                MigrationKind::Create(_) => {}
                MigrationKind::Modify(modifiers) => {
                    for modification in modifiers {
                        if !past_names.contains(&modification.key) {
                            continue;
                        }

                        match &modification.options {
                            ModifyColumnOptionsValues::Rename { to } => {
                                name = Some(to.to_string());
                                past_names.push(modification.key.clone());
                            }

                            ModifyColumnOptionsValues::Drop => name = None,
                            ModifyColumnOptionsValues::Add {
                                new_type: _,
                                new_options: _,
                            } => {}
                        }
                    }
                }
            }
        }

        name
    }

    /// This function returns all columns and types defined by the get_migration()
    fn get_columns() -> HashSet<(String, ColumnType)> {
        let migration = Self::get_migration();

        let mut columns: HashSet<(String, ColumnType)> = HashSet::new();

        for (idx, column) in migration
            .iter()
            .sorted_by_key(|item| item.ordering)
            .enumerate()
        {
            match &column.kind {
                MigrationKind::Create(values) => {
                    if idx != 0 {
                        panic!("The first iteration must be a creation.")
                    }

                    for item in values
                        .iter()
                        .map(|col| (Self::get_latest_column_name(&col.key).unwrap(), col.value))
                    {
                        columns.insert(item);
                    }
                }

                MigrationKind::Modify(values) => {
                    for value in values {
                        match value.options {
                            ModifyColumnOptionsValues::Rename { to: _ } => {}
                            ModifyColumnOptionsValues::Drop => {}
                            ModifyColumnOptionsValues::Add {
                                new_type,
                                new_options: _,
                            } => {
                                columns.insert((
                                    Self::get_latest_column_name(&value.key).unwrap(),
                                    new_type,
                                ));
                            }
                        }
                    }
                }
            }
        }

        if let MigrationKind::Create(value) = &migration[0].kind {
            for item in value
                .iter()
                .map(|e| (Self::get_latest_column_name(&e.key).unwrap(), e.value))
            {
                columns.insert(item);
            }
        }

        columns
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_migration(&self) -> &'static Vec<ModelMigration> {
        Self::get_migration()
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_table_name(&self) -> &'static str {
        Self::TABLE_NAME
    }

    /// This function is a helper intended for use in Box<dyn ...> situations where T is not available
    fn self_get_columns(&self) -> HashSet<(String, ColumnType)> {
        Self::get_columns()
    }
}
