use tokio_postgres::Row;

use crate::active_record_error::ActiveRecordError;
use crate::record::Record;
use crate::row_cursor::RowCursor;

pub(crate) fn read_record<Read: Record>(row: &Row) -> Result<Read, ActiveRecordError> {
    Read::read(&mut RowCursor::new(row, 0, Read::TABLE))
}
