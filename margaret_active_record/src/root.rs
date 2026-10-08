use crate::clause::Clause;
use crate::narrowed::Narrowed;
use crate::narrowing_state::NarrowingState;
use crate::selection::Selection;

#[derive(Clone, Copy)]
pub struct Root;

impl<Modeled> NarrowingState<Modeled> for Root {
    fn narrow(self, clause: Clause) -> Narrowed<Modeled> {
        Narrowed::new(clause)
    }

    fn selection(self) -> Selection {
        Selection::Everything
    }
}
