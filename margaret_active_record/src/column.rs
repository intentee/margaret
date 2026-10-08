use std::marker::PhantomData;
use std::sync::Arc;

use crate::assigned::Assigned;
use crate::assigned_value::AssignedValue;
use crate::assigning::Assigning;
use crate::conflicting::Conflicting;
use crate::field::Field;
use crate::field_columns::field_columns;
use crate::ordered::Ordered;
use crate::pending_assignment::PendingAssignment;
use crate::record::Record;
use crate::value::Value;

pub struct Column<Modeled, Held, Context> {
    field: PhantomData<fn(Modeled, Held) -> Context>,
    start: usize,
}

impl<Modeled: Record, Held: Field, Context> Column<Modeled, Held, Context> {
    #[must_use]
    pub fn new(start: usize) -> Self {
        Self {
            field: PhantomData,
            start,
        }
    }

    fn assigned(self, value: AssignedValue) -> Assigned<Modeled, Context> {
        Assigned::new(PendingAssignment {
            columns: field_columns(Modeled::TABLE, self.start, Held::Stored::WIDTH),
            value,
        })
    }
}

impl<Modeled: Record, Held: Field> Column<Modeled, Held, Assigning> {
    #[must_use]
    pub fn to(self, value: Held) -> Assigned<Modeled, Assigning> {
        self.assigned(AssignedValue::Given(Arc::new(value)))
    }
}

impl<Modeled: Record, Held: Field> Column<Modeled, Held, Conflicting> {
    #[must_use]
    pub fn excluded(self) -> Assigned<Modeled, Conflicting> {
        self.assigned(AssignedValue::Excluded)
    }
}

impl<Modeled: Record, Held: Ordered> Column<Modeled, Held, Conflicting> {
    #[must_use]
    pub fn greatest(self) -> Assigned<Modeled, Conflicting> {
        self.assigned(AssignedValue::Greatest)
    }
}
