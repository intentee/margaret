use chrono::DateTime;
use chrono::Utc;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;

macro_rules! scalar_value {
    ($scalar:ty) => {
        impl Value for $scalar {
            const WIDTH: usize = 1;

            fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
                cursor.read()
            }

            fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
                parameters.push(self.clone());

                Ok(())
            }
        }
    };
}

pub trait Value: Sized + Send + Sync + 'static {
    const WIDTH: usize;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the columns do not decode into the value.
    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError>;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the value cannot be encoded into its columns.
    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError>;
}

scalar_value!(bool);
scalar_value!(i32);
scalar_value!(i64);
scalar_value!(f32);
scalar_value!(f64);
scalar_value!(String);
scalar_value!(Vec<u8>);
scalar_value!(Uuid);
scalar_value!(DateTime<Utc>);
scalar_value!(Decimal);
