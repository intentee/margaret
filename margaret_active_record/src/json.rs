use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::active_record_error::ActiveRecordError;
use crate::parameters::Parameters;
use crate::row_cursor::RowCursor;
use crate::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Json<Payload> {
    payload: Payload,
}

impl<Payload> Json<Payload> {
    #[must_use]
    pub fn new(payload: Payload) -> Self {
        Self { payload }
    }

    #[must_use]
    pub fn into_payload(self) -> Payload {
        self.payload
    }

    #[must_use]
    pub fn payload(&self) -> &Payload {
        &self.payload
    }
}

impl<Payload: Serialize + DeserializeOwned + Send + Sync + 'static> Value for Json<Payload> {
    const WIDTH: usize = 1;

    fn read(cursor: &mut RowCursor<'_>) -> Result<Self, ActiveRecordError> {
        cursor.read_json().map(Self::new)
    }

    fn write(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        parameters.push_json(&self.payload)
    }
}
