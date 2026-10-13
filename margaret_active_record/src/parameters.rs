use serde::Serialize;
use tokio_postgres::types::ToSql;

use margaret_model::table::Table;
use margaret_sql::sql_parameter::SqlParameter;

use crate::active_record_error::ActiveRecordError;
use crate::null_parameter::NullParameter;

pub struct Parameters {
    table: &'static Table,
    values: Vec<SqlParameter>,
}

impl Parameters {
    pub(crate) fn new(table: &'static Table) -> Self {
        Self {
            table,
            values: Vec::new(),
        }
    }

    pub fn push(&mut self, value: impl ToSql + Send + Sync + 'static) {
        self.values.push(SqlParameter::new(value));
    }

    pub(crate) fn into_values(self) -> Vec<SqlParameter> {
        self.values
    }

    pub(crate) fn push_json(&mut self, payload: &impl Serialize) -> Result<(), ActiveRecordError> {
        serde_json::to_string(payload)
            .map(|serialized| self.push(serialized))
            .map_err(|source| ActiveRecordError::JsonSerialization {
                position: self.values.len(),
                source,
                table: self.table.name,
            })
    }

    pub(crate) fn push_nulls(&mut self, width: usize) {
        for _ in 0..width {
            self.push(NullParameter);
        }
    }

    pub(crate) fn push_parameter(&mut self, parameter: SqlParameter) {
        self.values.push(parameter);
    }
}
