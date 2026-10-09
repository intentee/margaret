use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;

use crate::active_record_error::ActiveRecordError;
use crate::key_target::KeyTarget;
use crate::parameters::Parameters;
use crate::record::Record;
use crate::row_cursor::RowCursor;
use crate::value::Value;

pub struct Key<Referenced: Record> {
    primary_key: Referenced::PrimaryKey,
}

impl<Referenced: Record> Key<Referenced> {
    #[must_use]
    pub fn new(primary_key: Referenced::PrimaryKey) -> Self {
        Self { primary_key }
    }

    #[must_use]
    pub fn into_primary_key(self) -> Referenced::PrimaryKey {
        self.primary_key
    }

    #[must_use]
    pub fn primary_key(&self) -> &Referenced::PrimaryKey {
        &self.primary_key
    }
}

impl<Referenced: KeyTarget> Key<Referenced> {
    #[must_use]
    pub fn of(record: &Referenced) -> Self {
        Self::new(record.primary_key())
    }
}

impl<Referenced: Record> Clone for Key<Referenced> {
    fn clone(&self) -> Self {
        Self::new(self.primary_key.clone())
    }
}

impl<Referenced: Record> Debug for Key<Referenced>
where
    Referenced::PrimaryKey: Debug,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Key")
            .field("primary_key", &self.primary_key)
            .finish()
    }
}

impl<Referenced: Record> Eq for Key<Referenced> where Referenced::PrimaryKey: Eq {}

impl<Referenced: Record> PartialEq for Key<Referenced>
where
    Referenced::PrimaryKey: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.primary_key == other.primary_key
    }
}

impl<Referenced: Record> Value for Key<Referenced>
where
    Referenced::PrimaryKey: Value,
{
    const WIDTH: usize = Referenced::PrimaryKey::WIDTH;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        Referenced::PrimaryKey::read(cursor).map(Self::new)
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        self.primary_key.write(parameters)
    }
}
