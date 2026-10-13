use std::marker::PhantomData;

use margaret_model::table::Table;
use margaret_sql::assignments::Assignments;

use crate::active_record_error::ActiveRecordError;
use crate::pending_assignment::PendingAssignment;

pub struct Assigned<Modeled, Context> {
    context: PhantomData<fn(Modeled) -> Context>,
    first: PendingAssignment,
    rest: Vec<PendingAssignment>,
}

impl<Modeled, Context> Assigned<Modeled, Context> {
    pub(crate) fn new(first: PendingAssignment) -> Self {
        Self {
            context: PhantomData,
            first,
            rest: Vec::new(),
        }
    }

    #[must_use]
    pub fn and(mut self, other: Self) -> Self {
        self.rest.push(other.first);
        self.rest.extend(other.rest);
        self
    }

    pub(crate) fn assignments(
        &self,
        table: &'static Table,
    ) -> Result<Assignments, ActiveRecordError> {
        self.first.assignment(table).and_then(|first| {
            self.rest
                .iter()
                .map(|pending| pending.assignment(table))
                .collect::<Result<Vec<_>, ActiveRecordError>>()
                .map(|rest| Assignments { first, rest })
        })
    }
}
