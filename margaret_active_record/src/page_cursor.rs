use std::marker::PhantomData;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;
use crate::raw_column::RawColumn;

pub struct PageCursor<Modeled, Ordering> {
    ordering: PhantomData<fn(Modeled) -> Ordering>,
    values: Vec<RawColumn>,
}

impl<Modeled, Ordering> PageCursor<Modeled, Ordering> {
    pub(crate) fn new(values: Vec<RawColumn>) -> Self {
        Self {
            ordering: PhantomData,
            values,
        }
    }
}

impl<Modeled, Ordering> Clone for PageCursor<Modeled, Ordering> {
    fn clone(&self) -> Self {
        Self::new(self.values.clone())
    }
}

impl<Modeled, Ordering> Encodable for PageCursor<Modeled, Ordering> {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        for value in &self.values {
            parameters.push(value.clone());
        }

        Ok(())
    }
}
