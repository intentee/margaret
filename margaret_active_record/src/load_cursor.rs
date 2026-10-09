use tokio_postgres::Row;

use crate::active_record_error::ActiveRecordError;
use crate::record::Record;
use crate::row_cursor::RowCursor;

pub struct LoadCursor<'row> {
    index: i64,
    offset: usize,
    row: &'row Row,
}

impl<'row> LoadCursor<'row> {
    pub(crate) fn new(row: &'row Row, offset: usize, index: i64) -> Self {
        Self { index, offset, row }
    }

    #[must_use]
    pub fn index(&self) -> i64 {
        self.index
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the columns do not decode into the record.
    pub fn record<Read: Record>(&mut self) -> Result<Read, ActiveRecordError> {
        Read::read(&mut RowCursor::new(self.row, self.offset, Read::TABLE)).inspect(|_| {
            self.offset += Read::TABLE.columns.len();
        })
    }

    pub fn skip(&mut self, width: usize) {
        self.offset += width;
    }

    pub(crate) fn next_columns_are_null(
        &self,
        width: usize,
        table: &'static margaret_model::table::Table,
    ) -> Result<bool, ActiveRecordError> {
        RowCursor::new(self.row, self.offset, table).next_columns_are_null(width)
    }
}
