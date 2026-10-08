use std::marker::PhantomData;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;
use crate::raw_column::RawColumn;

pub struct PageCursor<Modeled> {
    model: PhantomData<fn() -> Modeled>,
    values: Vec<RawColumn>,
}

impl<Modeled> PageCursor<Modeled> {
    pub(crate) fn new(values: Vec<RawColumn>) -> Self {
        Self {
            model: PhantomData,
            values,
        }
    }
}

impl<Modeled> Clone for PageCursor<Modeled> {
    fn clone(&self) -> Self {
        Self::new(self.values.clone())
    }
}

impl<Modeled> Encodable for PageCursor<Modeled> {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        for value in &self.values {
            parameters.push(value.clone());
        }

        Ok(())
    }
}
