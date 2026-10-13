use crate::active_record_error::ActiveRecordError;
use crate::field::Field;
use crate::parameters::Parameters;

pub trait Encodable: Send + Sync {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError>;
}

impl<Held: Field> Encodable for Held {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        self.write(parameters)
    }
}
