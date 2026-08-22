use crate::core::column::ModifyColumnOptionsValues;
use crate::core::column::create::CreateColumnOptionsValues;
use crate::core::column::create::CreateOptions;
use crate::core::column::create::CreateTableOptionValues;
use crate::core::search::constraint::OtherConstraint;
use crate::core::search::constraint::SearchConstraint;
use crate::core::search::search_op::SearchOp;
use crate::core::search::table_options::table_options_value::TableOptionsValue;
use crate::core::search::table_options::table_options_value::order_by_options::OrderByOptions;
use std::{
    any::{type_name, type_name_of_val},
    collections::HashSet,
};

use itertools::Itertools;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Transaction, params, params_from_iter};
use tracing::warn;
use tracing::{debug, error, info, trace};

use roxygen::roxygen;

use crate::{
    core::{
        MigrationKind,
        column::{ColumnType, ColumnValue},
        search::SearchQuery,
        traits::{
            from_iter::{FromIter, FromIterValue},
            model::Model,
            save_data::{SaveData, ValidateSaveData},
        },
    },
    server::database_strategy::{DatabaseStrategy, DatabaseStrategyError, TransactionOptions},
};

pub struct SqliteStrategy {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteStrategy {
    #[roxygen]
    /// Constructs a new SqliteStrategy from a path
    pub fn new(
        /// The path to use
        path: impl ToString,
    ) -> Self {
        let manager = SqliteConnectionManager::file(path.to_string());

        Self::init(manager)
    }

    /// Constructs a new SqliteStrategy using a :memory: database
    pub fn new_memory() -> Self {
        let manager = SqliteConnectionManager::memory();

        Self::init(manager)
    }

    fn init(manager: SqliteConnectionManager) -> Self {
        Self {
            pool: Pool::new(manager).unwrap(),
        }
    }

    fn match_column_type(value: &ColumnType) -> String {
        (match value {
            ColumnType::String => "TEXT",
            ColumnType::Integer => "INTEGER",
            ColumnType::Float => "REAL",
            ColumnType::Date => "TEXT",
            ColumnType::Json => "TEXT",
            ColumnType::Bool => "INTEGER",
        })
        .to_string()
    }

    fn match_create_column_options(value: &CreateOptions, _column_name: &str) -> String {
        let mut options = Vec::<String>::new();

        for (_, option) in value.column_options.iter().sorted_by_key(|value| value.0) {
            match option {
                CreateColumnOptionsValues::NonNullable => {
                    options.push("NOT NULL".to_string());
                }
                CreateColumnOptionsValues::PrimaryKey => {
                    options.push("PRIMARY KEY".to_string());
                }
                CreateColumnOptionsValues::Default(default) => {
                    options.push(format!("DEFAULT {default}"));
                }
                CreateColumnOptionsValues::Unique => {
                    options.push("UNIQUE".to_string());
                }
                CreateColumnOptionsValues::Check(check) => {
                    options.push(format!("CHECK({check})"));
                }
            }
        }

        options.join("  ")
    }

    fn match_modify_column_options(
        value: &crate::core::column::ModifyColumnOptionsValues,
        column_name: &str,
    ) -> String {
        let mut out = String::new();
        match value {
            ModifyColumnOptionsValues::Rename { to } => {
                out.push_str(&format!("RENAME COLUMN {column_name} TO {to}"));
            }
            ModifyColumnOptionsValues::Drop => {
                out.push_str(&format!("DROP COLUMN {column_name}"));
            }
            ModifyColumnOptionsValues::Add {
                new_type,
                new_options,
            } => {
                out.push_str(&format!(
                    "ADD COLUMN {column_name} {} {}",
                    SqliteStrategy::match_column_type(new_type),
                    SqliteStrategy::match_create_column_options(new_options, column_name)
                ));
            }
        }

        out
    }

    fn match_column_value(value: &ColumnValue) -> String {
        match value {
            ColumnValue::String(value) => value.clone(),
            ColumnValue::Integer(value) => format!("{value}"),
            ColumnValue::Float(value) => format!("{value:.4}"),
            ColumnValue::Date(value) => value.to_rfc3339(),
            ColumnValue::Json(value) => value.clone(),
            ColumnValue::Null => "null".to_string(),
        }
    }

    fn match_create_table_options(
        value: &HashSet<CreateTableOptionValues>,
        column_name: &str,
    ) -> String {
        let mut options = Vec::<String>::new();

        for option in value.iter() {
            match option {
                CreateTableOptionValues::ForeignKey { table, column } => {
                    options.push(format!(
                        "FOREIGN KEY ({column_name}) REFERENCES {table}({column})"
                    ));
                }
            }
        }

        options.join(",\n")
    }

    fn parse_constraints<T>(input: Option<SearchConstraint>) -> (String, Vec<ColumnValue>)
    where
        T: Model,
    {
        let mut output = String::new();
        let mut iterations = 0;
        let mut values: Vec<ColumnValue> = Vec::new();

        let mut next = input;

        while let Some(value) = next {
            iterations += 1;
            let (column, operator, value, other) = value.get();
            let column = T::get_latest_column_name(&column).unwrap();

            values.push(value);

            output += &format!(
                "{column} {} ?{}",
                match operator {
                    SearchOp::EQ => "=",
                    SearchOp::NEQ => "!=",
                    SearchOp::GT => ">",
                    SearchOp::GTEQ => ">=",
                    SearchOp::LT => "<",
                    SearchOp::LTEQ => "<=",
                },
                iterations
            );

            if iterations % 2 == 0 {
                output = format!("({output})");
            }

            if let Some(other) = other {
                next = match *other {
                    OtherConstraint::And(search_constraint) => {
                        output += " AND ";
                        Some(search_constraint)
                    }
                    OtherConstraint::Or(search_constraint) => {
                        output += " OR  ";
                        Some(search_constraint)
                    }
                };
            } else {
                next = None;
            }
        }

        (output, values)
    }
}

impl DatabaseStrategy for SqliteStrategy {
    type ConnectionType<'a> = PooledConnection<SqliteConnectionManager>;
    type TransactionType<'a> = Transaction<'a>;

    type FunctionConnType<'a>
        = Connection
    where
        Self: 'a;

    fn get_connection(&self) -> Self::ConnectionType<'_> {
        self.pool.get().unwrap()
    }

    fn with_transaction<F, T>(&self, function: F) -> Result<T, DatabaseStrategyError>
    where
        F: FnOnce(Self::TransactionType<'_>) -> T,
    {
        let mut conn = self.get_connection();
        let transaction = conn
            .transaction()
            .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        Ok(function(transaction))
    }

    fn migrate_model<M: Model>(&self) -> Result<(), DatabaseStrategyError> {
        let migration_data = M::get_migration();

        let table_name = M::TABLE_NAME;

        if migration_data.is_empty() {
            return Err(DatabaseStrategyError::MigrateModel(format!(
                "Migration for model {} needs to have at least one migration!",
                table_name
            )));
        }

        let mut pool = self.get_connection();
        let transaction = pool
            .transaction()
            .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        for (count, migration) in migration_data
            .iter()
            .sorted_by_key(|item| item.ordering)
            .enumerate()
        {
            self.setup_migration_table(&transaction)?;
            if let Some(migration) = self.get_last_migration(&transaction, table_name)?
                && migration >= count as i64
            {
                debug!(
                    "{}: migration {} not needed, idx {}",
                    table_name, migration, count,
                );
                continue;
            }

            match &migration.kind {
                MigrationKind::Create(columns) => {
                    if count != 0 {
                        return Err(DatabaseStrategyError::MigrateModel(format!(
                            "Can only create a table at the first iteration, not at iteration {count}"
                        )));
                    }

                    if self.table_exists(&transaction, table_name)? {
                        continue;
                    }

                    let mut sql = format!("CREATE TABLE {} (\n", table_name);
                    sql += &columns
                        .iter()
                        .map(|col| {
                            format!(
                                "\t{} {} {}",
                                col.key,
                                SqliteStrategy::match_column_type(&col.value),
                                SqliteStrategy::match_create_column_options(&col.options, &col.key)
                            )
                        })
                        .join(",\n");

                    let create_table_sql = &columns
                        .iter()
                        .filter(|col| !col.options.table_options.is_empty())
                        .map(|col| {
                            format!(
                                "\t{}",
                                SqliteStrategy::match_create_table_options(
                                    &col.options.table_options,
                                    &col.key
                                )
                            )
                        })
                        .join(",\n");

                    if !create_table_sql.is_empty() {
                        sql += &format!(",\n{}", create_table_sql);
                    }

                    sql += "\n)";

                    trace!("produced sql: {sql}");

                    transaction
                        .execute(&sql, [])
                        .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;

                    info!("Created table {}", M::TABLE_NAME);
                }
                MigrationKind::Modify(columns) => {
                    for col in columns {
                        let mut sql = format!("ALTER TABLE {table_name}\n");
                        sql += &SqliteStrategy::match_modify_column_options(&col.options, &col.key)
                            .to_string();

                        sql += ";";
                        trace!("produced sql: {sql}");

                        transaction
                            .execute(&sql, [])
                            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))?;
                    }
                }
            }

            self.on_migration_run(&transaction, table_name, count as i64)?;
        }

        transaction
            .commit()
            .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        Ok(())
    }

    fn table_exists(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
    ) -> Result<bool, DatabaseStrategyError> {
        conn.table_exists(None, table_name)
            .map_err(|e| DatabaseStrategyError::MigrateModel(e.to_string()))
    }

    fn setup_migration_table(
        &self,
        conn: &Self::FunctionConnType<'_>,
    ) -> Result<(), DatabaseStrategyError> {
        if self.table_exists(conn, "_migrations")? {
            return Ok(());
        }

        let sql = "CREATE TABLE _migrations (
            table_name TEXT NOT NULL,
            last_migration INTEGER NOT NULL
        )";

        conn.execute(sql, [])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn on_migration_run(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
        count: i64,
    ) -> Result<(), DatabaseStrategyError> {
        let sql = "INSERT INTO _migrations (table_name, last_migration) values (?1, ?2)";
        conn.execute(sql, params![table_name, count])
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        Ok(())
    }

    fn get_last_migration(
        &self,
        conn: &Self::FunctionConnType<'_>,
        table_name: &str,
    ) -> Result<Option<i64>, DatabaseStrategyError> {
        let sql = "select * from _migrations where table_name = ?1";
        let mut result = conn
            .prepare(sql)
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result
            .query_map(params![table_name], |row| {
                row.get("last_migration").map(|e: i64| e)
            })
            .map_err(|e| DatabaseStrategyError::MigrationTable(e.to_string()))?;

        let result = result.into_iter().flatten().max().map(Some).unwrap_or(None);

        Ok(result)
    }

    fn save_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        model: &mut T,
    ) -> Result<(), DatabaseStrategyError>
    where
        T: SaveData + Model + FromIter + ValidateSaveData,
    {
        let data = model.get_save_data();
        let table_name = T::TABLE_NAME;

        let model_name = type_name_of_val(model);

        if !self.table_exists(conn, table_name)? {
            return Err(DatabaseStrategyError::SaveModel {
                err: format!(
                    "Table '{table_name}' does not exist. Did you forget to call 'migrate_model'?"
                ),
                model: model_name,
            });
        }

        if let Some(missing_data) = model.validate_save_data() {
            return Err(DatabaseStrategyError::SaveModel {
                err: format!(
                    "Missing the following columns in the save_data: {}",
                    missing_data.join(",")
                ),
                model: model_name,
            });
        }

        let mut sql = String::new();

        let columns_values = data
            .iter()
            .filter(|e| !matches!(e.value, ColumnValue::Null))
            .map(|e| {
                let value = Self::match_column_value(&e.value);

                (e.key.clone(), value)
            })
            .collect_vec();

        let model_id_column = model.get_id_column_name();

        if let Some(model_id) = model.get_id() {
            let columns_values = columns_values
                .into_iter()
                .filter(|(column, _)| column != model_id_column)
                .collect_vec();

            sql += &format!("UPDATE {table_name} SET ");

            sql += &columns_values
                .iter()
                .enumerate()
                .map(|(index, (column, _))| format!("{column} = ?{}", index + 1))
                .join(", ");

            sql += &format!(" WHERE {} = {}", model_id_column, model_id);

            trace!("Generated update sql: {sql}");
            conn.execute(
                &sql,
                params_from_iter(columns_values.iter().map(|(_, value)| value)),
            )
            .map_err(|e| DatabaseStrategyError::SaveModel {
                err: e.to_string(),
                model: type_name_of_val(model),
            })?;
        } else {
            trace!("columns: {:?}", columns_values);
            sql += &format!(
                "INSERT INTO {table_name} ({}) VALUES ({})",
                columns_values.iter().map(|(column, _)| column).join(", "),
                (1..columns_values.len() + 1)
                    .map(|i| format!("?{i}"))
                    .join(", ")
            );

            sql += &(" RETURNING ".to_string() + model_id_column);

            trace!("Generated insert sql: {sql}");

            let id = conn
                .query_one(
                    &sql,
                    params_from_iter(columns_values.iter().map(|(_, value)| value)),
                    |e| e.get(model_id_column).map(|e: i64| e),
                )
                .map_err(|e| DatabaseStrategyError::SaveModel {
                    err: e.to_string(),
                    model: type_name_of_val(model),
                })?;

            model.set_id(id);
        }

        Ok(())
    }

    fn search_single_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        mut query: SearchQuery,
    ) -> Result<Option<T>, DatabaseStrategyError>
    where
        T: Model + FromIter,
        Self: Sized,
    {
        unsafe {
            query.set_limit(1);
        }
        let mut models = self.search_multiple_model(conn, query)?;

        if !models.is_empty() {
            return Ok(Some(models.remove(0)));
        }

        Ok(None)
    }

    fn search_multiple_model<T>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        query: SearchQuery,
    ) -> Result<Vec<T>, DatabaseStrategyError>
    where
        T: Model + FromIter,
    {
        let mut sql = String::new();
        let table_name = T::TABLE_NAME;

        let (constraints, table_options) = query.values();

        sql += &format!("SELECT * FROM {table_name}");

        let (constraints, values) = Self::parse_constraints::<T>(constraints);

        if !constraints.is_empty() {
            sql += &format!(" WHERE {constraints}");
        }

        if let Some(table_options) = table_options {
            let table_options = table_options.values();

            for option in table_options
                .into_iter()
                .sorted_by_key(Self::table_options_priority)
            {
                match option {
                    TableOptionsValue::Limit(limit) => {
                        sql += &format!(" LIMIT {limit}");
                    }
                    TableOptionsValue::OrderBy { column, options } => {
                        let column = T::get_latest_column_name(&column).unwrap();

                        sql += &format!(
                            " Order by {column} {}",
                            match options {
                                Some(value) => {
                                    match value {
                                        OrderByOptions::Asc => "ASC",
                                        OrderByOptions::Desc => "DESC",
                                    }
                                }
                                None => "",
                            }
                        )
                    }
                }
            }
        }

        let params = values
            .into_iter()
            .map(|e| Self::match_column_value(&e))
            .collect_vec();

        trace!("Generated sql: {sql} | params: {:?}", &params);

        let columns = T::get_columns();

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| DatabaseStrategyError::Error(e.to_string()))?;

        let rows = stmt
            .query_map(params_from_iter(params.into_iter()), |row| {
                let iter = columns.iter().map(|(column, column_type)| {
                    let value = match column_type {
                        ColumnType::Integer => {
                            row.get(column.as_str()).map(|e: i64| format!("{e}"))
                        }
                        ColumnType::Float => row.get(column.as_str()).map(|e: f64| format!("{e}")),
                        ColumnType::String |
                        ColumnType::Date |
                        ColumnType::Json |
                        ColumnType::Bool => row.get(column.as_str()).map(|e: String| e),
                    };

                    let value = match value {
                        Ok(value) => {
                            value
                        },
                        Err(e) => {
                            error!(
                                "Expected Column {column} with type {column_type:?} on Model {}, error: {e:?}",
                                type_name::<T>()
                            );
                            return None;
                        }
                    };

                    Some(FromIterValue {
                        column_name: column.to_string(),
                        column_value: value,
                        column_type: *column_type
                    })
                });

                match iter.clone().all(|value| value.is_some()) {
                    true => {
                        let value = T::from_iter(iter.map(|value| value.unwrap()));

                        debug!("Got to parse value| is_some: {}", value.is_some());
                        Ok(value)
                    },
                    false => {
                        warn!("Failed to test for all Some() values");
                        Ok(None)
                    }
                }

            })
            .map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;

        let mut rows_ret: u64 = 0;

        let models = {
            let mut models = Vec::new();
            for row in rows {
                rows_ret += 1;
                let row = row.map_err(|e| DatabaseStrategyError::SearchModel(e.to_string()))?;
                match row {
                    Some(value) => {
                        models.push(value);
                    }
                    None => {
                        trace!("Something was none");
                    }
                }
            }
            models
        };

        trace!(
            "Database returned {} rows, {} parsed models",
            rows_ret,
            models.len()
        );

        Ok(models)
    }

    fn remove_model<T: Model>(
        &self,
        conn: &Self::FunctionConnType<'_>,
        query: SearchQuery,
    ) -> Result<(), DatabaseStrategyError> {
        let table_name = T::TABLE_NAME;

        let mut sql = String::new();

        sql += &format!("DELETE FROM {table_name}");

        let (constraints, _) = query.values();

        let (constraints, values) = Self::parse_constraints::<T>(constraints);

        if !constraints.is_empty() {
            sql += &format!(" WHERE {constraints}")
        }

        trace!("Generated sql: {sql}");

        let params = values
            .into_iter()
            .map(|e| Self::match_column_value(&e))
            .collect_vec();

        conn.execute(&sql, params_from_iter(params))
            .map_err(|e| DatabaseStrategyError::DeleteModel(e.to_string()))?;

        Ok(())
    }

    fn manage_transaction(
        &self,
        conn: Self::TransactionType<'_>,
        options: TransactionOptions,
    ) -> Result<(), DatabaseStrategyError> {
        match options {
            TransactionOptions::Commit => conn.commit(),
            TransactionOptions::Rollback => conn.rollback(),
        }
        .map_err(|e| DatabaseStrategyError::Transaction(e.to_string()))?;

        Ok(())
    }
}
