use std::marker::PhantomData;
use std::sync::Arc;

use margaret_sql::comparison::Comparison;

use crate::clause::Clause;
use crate::field::Field;
use crate::field_columns::field_columns;
use crate::ordered::Ordered;
use crate::predicate::Predicate;
use crate::record::Record;
use crate::value::Value;

pub struct Operand<Modeled, Held> {
    pub field: PhantomData<fn(Modeled) -> Held>,
    pub start: usize,
}

impl<Modeled: Record, Held: Field> Operand<Modeled, Held> {
    #[must_use]
    pub fn eq(self, value: Held::Stored) -> Predicate<Modeled> {
        self.compared(Comparison::Equal, value)
    }

    fn compared(self, comparison: Comparison, value: Held::Stored) -> Predicate<Modeled> {
        Predicate::new(Clause::Compare {
            columns: field_columns(Modeled::TABLE, self.start, Held::Stored::WIDTH),
            comparison,
            value: Arc::new(value),
        })
    }
}

impl<Modeled: Record, Held: Field> Operand<Modeled, Held>
where
    Held::Stored: Ordered,
{
    #[must_use]
    pub fn above(self, bound: Held::Stored) -> Predicate<Modeled> {
        self.compared(Comparison::Greater, bound)
    }

    #[must_use]
    pub fn at_least(self, bound: Held::Stored) -> Predicate<Modeled> {
        self.compared(Comparison::GreaterOrEqual, bound)
    }

    #[must_use]
    pub fn at_most(self, bound: Held::Stored) -> Predicate<Modeled> {
        self.compared(Comparison::LessOrEqual, bound)
    }

    #[must_use]
    pub fn below(self, bound: Held::Stored) -> Predicate<Modeled> {
        self.compared(Comparison::Less, bound)
    }
}

impl<Modeled: Record, Stored: Value> Operand<Modeled, Option<Stored>> {
    #[must_use]
    pub fn is_null(self) -> Predicate<Modeled> {
        Predicate::new(Clause::IsNull {
            columns: field_columns(Modeled::TABLE, self.start, Stored::WIDTH),
        })
    }
}
