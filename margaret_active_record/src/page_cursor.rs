use std::marker::PhantomData;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::parameters::Parameters;
use crate::raw_column::RawColumn;

pub struct PageCursor<Modeled, Ordering, Toward> {
    ordering: PhantomData<fn(Modeled, Toward) -> Ordering>,
    values: Vec<RawColumn>,
}

impl<Modeled, Ordering, Toward> PageCursor<Modeled, Ordering, Toward> {
    pub(crate) fn new(values: Vec<RawColumn>) -> Self {
        Self {
            ordering: PhantomData,
            values,
        }
    }
}

impl<Modeled, Ordering, Toward> Clone for PageCursor<Modeled, Ordering, Toward> {
    fn clone(&self) -> Self {
        Self::new(self.values.clone())
    }
}

impl<Modeled, Ordering, Toward> Encodable for PageCursor<Modeled, Ordering, Toward> {
    fn encode(&self, parameters: &mut Parameters) -> Result<(), ActiveRecordError> {
        for value in &self.values {
            parameters.push(value.clone());
        }

        Ok(())
    }
}
