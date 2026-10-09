use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;

pub trait DraftRecord: Send + Sync + 'static {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when a field cannot be encoded into its columns.
    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError>;
}
