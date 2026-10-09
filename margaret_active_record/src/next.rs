use std::marker::PhantomData;
use std::sync::Arc;

use margaret_sql::comparison::Comparison;
use margaret_sql::direction::Direction;

use crate::clause::Clause;
use crate::continuation::Continuation as _;
use crate::field::Field;
use crate::field_columns::field_columns;
use crate::narrowed::Narrowed;
use crate::narrowing_state::NarrowingState;
use crate::ordered::Ordered;
use crate::query_edge::QueryEdge;
use crate::ranged::Ranged;
use crate::record::Record;
use crate::scan::Scan;
use crate::scan_order::ScanOrder;
use crate::value::Value;

pub struct Next<Modeled, Held, Edge, State> {
    field: PhantomData<fn(Modeled, Held) -> Edge>,
    start: usize,
    state: State,
}

impl<Modeled: Record, Held: Field, Edge: QueryEdge<Modeled>>
    Next<Modeled, Held, Edge, Narrowed<Modeled>>
{
    #[must_use]
    pub fn into_narrowed(self) -> Narrowed<Modeled> {
        self.state
    }
}

impl<Modeled, Held, Edge, State> Next<Modeled, Held, Edge, State>
where
    Modeled: Record,
    Held: Field,
    Edge: QueryEdge<Modeled>,
    State: NarrowingState<Modeled>,
{
    #[must_use]
    pub const fn new(state: State, start: usize) -> Self {
        Self {
            field: PhantomData,
            start,
            state,
        }
    }

    #[must_use]
    pub fn eq(self, value: Held::Stored) -> Edge::Continued {
        Edge::Continued::continued(self.narrowed(Comparison::Equal, value))
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

impl<Modeled, Held, Edge, State> Next<Modeled, Held, Edge, State>
where
    Modeled: Record,
    Held: Field,
    Held::Stored: Ordered,
    Edge: QueryEdge<Modeled>,
    State: NarrowingState<Modeled>,
{
    #[must_use]
    pub fn above(self, bound: Held::Stored) -> Ranged<Modeled, Edge> {
        self.ranged(Comparison::Greater, bound)
    }

    #[must_use]
    pub fn at_least(self, bound: Held::Stored) -> Ranged<Modeled, Edge> {
        self.ranged(Comparison::GreaterOrEqual, bound)
    }

    #[must_use]
    pub fn at_most(self, bound: Held::Stored) -> Ranged<Modeled, Edge> {
        self.ranged(Comparison::LessOrEqual, bound)
    }

    #[must_use]
    pub fn below(self, bound: Held::Stored) -> Ranged<Modeled, Edge> {
        self.ranged(Comparison::Less, bound)
    }

    fn ranged(self, comparison: Comparison, bound: Held::Stored) -> Ranged<Modeled, Edge> {
        Ranged::new(self.narrowed(comparison, bound))
    }
}

impl<Modeled, Held, Edge, State> Next<Modeled, Held, Edge, State>
where
    Modeled: Record,
    Held: Value,
    Edge: QueryEdge<Modeled>,
    State: NarrowingState<Modeled>,
    Edge: ScanOrder<Modeled>,
{
    #[must_use]
    pub fn ascending(self) -> Scan<Modeled, Edge> {
        self.scanned(Direction::Ascending)
    }

    #[must_use]
    pub fn descending(self) -> Scan<Modeled, Edge> {
        self.scanned(Direction::Descending)
    }

    fn scanned(self, direction: Direction) -> Scan<Modeled, Edge> {
        Scan::new(self.state.selection(), direction)
    }
}
