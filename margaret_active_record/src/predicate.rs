use std::marker::PhantomData;

use crate::clause::Clause;

pub struct Predicate<Row> {
    pub(crate) clause: Clause,
    row: PhantomData<fn() -> Row>,
}

impl<Row> Predicate<Row> {
    pub(crate) fn new(clause: Clause) -> Self {
        Self {
            clause,
            row: PhantomData,
        }
    }

    #[must_use]
    pub fn and(self, other: Self) -> Self {
        Self::new(self.clause.and(other.clause))
    }

    #[must_use]
    pub fn negated(self) -> Self {
        Self::new(Clause::Not(Box::new(self.clause)))
    }
}
