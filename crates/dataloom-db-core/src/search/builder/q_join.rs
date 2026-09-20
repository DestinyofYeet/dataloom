use crate::{
    search::{builder::SearchQueryBuilder, join_options::JoinOptions},
    traits::model::Model,
};

impl SearchQueryBuilder {
    pub fn q_join<M>(
        mut self,
        join_on_own_field: impl Into<String>,
        join_on_root_table_field: impl Into<String>,
    ) -> Self
    where
        M: Model,
    {
        self.join_options.push(JoinOptions {
            table_name: M::TABLE_NAME.to_string(),
            join_on_own_field: join_on_own_field.into(),
            join_on_root_table_field: join_on_root_table_field.into(),
        });
        self
    }
}
