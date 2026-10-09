use crate::clause::Clause;
use crate::narrowed::Narrowed;
use crate::selection::Selection;

pub trait NarrowingState<Modeled> {
    fn narrow(self, clause: Clause) -> Narrowed<Modeled>;

    fn selection(self) -> Selection;
}
