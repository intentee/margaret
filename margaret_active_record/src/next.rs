use std::marker::PhantomData;
use std::sync::Arc;

use margaret_sql::comparison::Comparison;
use margaret_sql::direction::Direction;

use crate::clause::Clause;
use crate::continuation::Continuation;
use crate::field::Field;
use crate::field_columns::field_columns;
use crate::narrowed::Narrowed;
use crate::narrowing_state::NarrowingState;
use crate::ordered::Ordered;
use crate::ordering_columns::ordering_columns;
use crate::ranged::Ranged;
use crate::record::Record;
use crate::root::Root;
use crate::scan::Scan;
use crate::scan_order::ScanOrder;
use crate::value::Value;

pub struct Next<Modeled, Held, Continued, State> {
    field: PhantomData<fn(Modeled, Held) -> Continued>,
    start: usize,
    state: State,
}

impl<Modeled: Record, Held: Field, Continued: Continuation<Modeled>>
    Next<Modeled, Held, Continued, Root>
{
    #[must_use]
    pub fn root(start: usize) -> Self {
        Self {
            field: PhantomData,
            start,
            state: Root,
        }
    }
}

impl<Modeled: Record, Held: Field, Continued: Continuation<Modeled>>
    Next<Modeled, Held, Continued, Narrowed<Modeled>>
{
    #[must_use]
    pub fn after(narrowed: Narrowed<Modeled>, start: usize) -> Self {
        Self {
            field: PhantomData,
            start,
            state: narrowed,
        }
    }

    #[must_use]
    pub fn into_narrowed(self) -> Narrowed<Modeled> {
        self.state
    }
}

impl<Modeled, Held, Continued, State> Next<Modeled, Held, Continued, State>
where
    Modeled: Record,
    Held: Field,
    Continued: Continuation<Modeled>,
    State: NarrowingState<Modeled>,
{
    #[must_use]
    pub fn eq(self, value: Held::Stored) -> Continued {
        Continued::continued(self.narrowed(Comparison::Equal, value))
    }

    fn columns(&self) -> Vec<&'static str> {
        field_columns(Modeled::TABLE, self.start, Held::Stored::WIDTH)
    }

    fn narrowed(self, comparison: Comparison, value: Held::Stored) -> Narrowed<Modeled> {
        let columns = self.columns();

        self.state.narrow(Clause::Compare {
            columns,
            comparison,
            value: Arc::new(value),
        })
    }
}

impl<Modeled, Held, Continued, State> Next<Modeled, Held, Continued, State>
where
    Modeled: Record,
    Held: Field,
    Held::Stored: Ordered,
    Continued: Continuation<Modeled>,
    State: NarrowingState<Modeled>,
{
    #[must_use]
    pub fn above(self, bound: Held::Stored) -> Ranged<Modeled, Continued> {
        self.ranged(Comparison::Greater, bound)
    }

    #[must_use]
    pub fn at_least(self, bound: Held::Stored) -> Ranged<Modeled, Continued> {
        self.ranged(Comparison::GreaterOrEqual, bound)
    }

    #[must_use]
    pub fn at_most(self, bound: Held::Stored) -> Ranged<Modeled, Continued> {
        self.ranged(Comparison::LessOrEqual, bound)
    }

    #[must_use]
    pub fn below(self, bound: Held::Stored) -> Ranged<Modeled, Continued> {
        self.ranged(Comparison::Less, bound)
    }

    fn ranged(self, comparison: Comparison, bound: Held::Stored) -> Ranged<Modeled, Continued> {
        let head = self.columns();

        Ranged::new(self.narrowed(comparison, bound), head)
    }
}

impl<Modeled, Held, Continued, State> Next<Modeled, Held, Continued, State>
where
    Modeled: Record,
    Held: Value,
    Continued: Continuation<Modeled> + ScanOrder<Modeled>,
    State: NarrowingState<Modeled>,
{
    #[must_use]
    pub fn ascending(self) -> Scan<Modeled> {
        self.scanned(Direction::Ascending)
    }

    #[must_use]
    pub fn descending(self) -> Scan<Modeled> {
        self.scanned(Direction::Descending)
    }

    fn scanned(self, direction: Direction) -> Scan<Modeled> {
        let head = self.columns();

        Scan::new(
            self.state.selection(),
            ordering_columns(Modeled::TABLE, head, Continued::REST),
            direction,
        )
    }
}
