#[derive(Debug, Clone)]
pub struct JoinOptions {
    pub table_name: String,
    pub join_on_own_field: String,
    pub join_on_root_table_field: String,
}
