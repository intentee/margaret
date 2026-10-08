use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;
use crate::record::Record;

pub struct RecordEncoding<'record, Encoded> {
    record: &'record Encoded,
}

impl<'record, Encoded: Record> RecordEncoding<'record, Encoded> {
    pub(crate) fn new(record: &'record Encoded) -> Self {
        Self { record }
    }
}

impl<Encoded: Record> Encodable for RecordEncoding<'_, Encoded> {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        self.record.write(parameters)
    }
}
