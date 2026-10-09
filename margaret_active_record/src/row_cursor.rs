use serde::de::DeserializeOwned;
use tokio_postgres::Row;
use tokio_postgres::types::FromSqlOwned;

use margaret_model::table::Table;

use crate::active_record_error::ActiveRecordError;
use crate::column_presence::ColumnPresence;
use crate::row_column::row_column;

pub struct RowCursor<'row> {
    column: usize,
    offset: usize,
    row: &'row Row,
    table: &'static Table,
}

impl<'row> RowCursor<'row> {
    pub(crate) fn new(row: &'row Row, offset: usize, table: &'static Table) -> Self {
        Self {
            column: 0,
            offset,
            row,
            table,
        }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError::MalformedColumn` when the column does not decode into the
    /// value.
    pub fn read<Read: FromSqlOwned>(&mut self) -> Result<Read, ActiveRecordError> {
        let position = self.offset + self.column;

        self.column += 1;
        row_column(self.row, position, self.table)
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError::UnknownVariant` when the stored text names no variant.
    pub fn read_variant<Variant>(
        &mut self,
        enum_type: &'static str,
        decode: fn(&str) -> Option<Variant>,
    ) -> Result<Variant, ActiveRecordError> {
        let position = self.offset + self.column;
        let table = self.table.name;

        self.read::<String>()
            .and_then(|stored| match decode(&stored) {
                Some(variant) => Ok(variant),
                None => Err(ActiveRecordError::UnknownVariant {
                    enum_type,
                    position,
                    stored,
                    table,
                }),
            })
    }

    pub(crate) fn read_json<Payload: DeserializeOwned>(
        &mut self,
    ) -> Result<Payload, ActiveRecordError> {
        let position = self.offset + self.column;
        let table = self.table.name;

        self.read::<String>().and_then(|stored| {
            serde_json::from_str(&stored).map_err(|source| ActiveRecordError::MalformedJson {
                position,
                source,
                table,
            })
        })
    }

    pub(crate) fn next_columns_are_null(&self, width: usize) -> Result<bool, ActiveRecordError> {
        let first = self.offset + self.column;

        (first..first + width).try_fold(true, |all_null, position| {
            row_column::<ColumnPresence>(self.row, position, self.table)
                .map(|presence| all_null && matches!(presence, ColumnPresence::Null))
        })
    }

    pub(crate) fn skip(&mut self, width: usize) {
        self.column += width;
    }
}
