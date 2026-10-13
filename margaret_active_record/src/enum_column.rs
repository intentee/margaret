use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;
use crate::value::Value;

pub trait EnumColumn: Sized + Send + Sync + 'static {
    const ENUM_TYPE: &'static str;

    fn from_variant_name(stored: &str) -> Option<Self>;

    fn variant_name(&self) -> &'static str;
}

impl<Variant: EnumColumn> Value for Variant {
    const WIDTH: usize = 1;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        cursor.read_variant(Self::ENUM_TYPE, Self::from_variant_name)
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        parameters.push(self.variant_name());

        Ok(())
    }
}
