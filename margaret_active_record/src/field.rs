use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;
use crate::value::Value;

pub trait Field: Sized + Send + Sync + 'static {
    type Stored: Value;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the columns do not decode into the field.
    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError>;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the field cannot be encoded into its columns.
    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError>;
}

impl<Stored: Value> Field for Stored {
    type Stored = Stored;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        Value::read(cursor)
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        Value::write(self, parameters)
    }
}

impl<Stored: Value> Field for Option<Stored> {
    type Stored = Stored;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        cursor
            .next_columns_are_null(Stored::WIDTH)
            .and_then(|null| {
                if null {
                    cursor.skip(Stored::WIDTH);

                    Ok(None)
                } else {
                    Stored::read(cursor).map(Some)
                }
            })
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        if let Some(stored) = self {
            stored.write(parameters)
        } else {
            parameters.push_nulls(Stored::WIDTH);

            Ok(())
        }
    }
}
