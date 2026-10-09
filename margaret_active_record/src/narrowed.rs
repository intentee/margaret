use std::marker::PhantomData;

use crate::clause::Clause;
use crate::narrowing_state::NarrowingState;
use crate::selection::Selection;

pub struct Narrowed<Modeled> {
    pub(crate) clause: Clause,
    model: PhantomData<fn() -> Modeled>,
}

impl<Modeled> Narrowed<Modeled> {
    pub(crate) fn new(clause: Clause) -> Self {
        Self {
            clause,
            model: PhantomData,
        }
    }
}

impl<Modeled> Clone for Narrowed<Modeled> {
    fn clone(&self) -> Self {
        Self::new(self.clause.clone())
    }
}

impl<Modeled> NarrowingState<Modeled> for Narrowed<Modeled> {
    fn narrow(self, clause: Clause) -> Narrowed<Modeled> {
        Self::new(self.clause.and(clause))
    }

    fn selection(self) -> Selection {
        Selection::Matching(self.clause)
    }
}
