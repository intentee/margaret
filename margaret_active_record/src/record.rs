use margaret_model::table::Table;

use crate::active_record_error::ActiveRecordError;
use crate::field_span::FieldSpan;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;

pub trait Record: Sized + Send + Sync + 'static {
    type PrimaryKey: Clone + Send + Sync + 'static;

    const PRIMARY_KEY: &'static [FieldSpan];

    const TABLE: &'static Table;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row does not decode into the record.
    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError>;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when a field cannot be encoded into its columns.
    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError>;
}
