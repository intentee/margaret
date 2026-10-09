use std::marker::PhantomData;

use margaret_database::executor::Executor;
use margaret_sql::direction::Direction;

use crate::active_record_error::ActiveRecordError;
use crate::assignable::Assignable;
use crate::assigned::Assigned;
use crate::assigning::Assigning;
use crate::bulk_delete::bulk_delete;
use crate::bulk_update::bulk_update;
use crate::field_set::FieldSet;
use crate::model::Model;
use crate::narrowed::Narrowed;
use crate::ordering_columns::ordering_columns;
use crate::record::Record;
use crate::scan::Scan;
use crate::scan_order::ScanOrder;
use crate::selection::Selection;

pub struct Ranged<Modeled, Continued> {
    continued: PhantomData<fn() -> Continued>,
    head: Vec<&'static str>,
    narrowed: Narrowed<Modeled>,
}

impl<Modeled: Record, Continued> Ranged<Modeled, Continued> {
    pub(crate) fn new(narrowed: Narrowed<Modeled>, head: Vec<&'static str>) -> Self {
        Self {
            continued: PhantomData,
            head,
            narrowed,
        }
    }
}

impl<Modeled: Model, Continued> Ranged<Modeled, Continued> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the rows cannot be deleted.
    pub async fn delete<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<u64, ActiveRecordError> {
        bulk_delete(self.narrowed.clause, Modeled::TABLE, executor).await
    }
}

impl<Modeled: Assignable, Continued> Ranged<Modeled, Continued> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the rows cannot be updated.
    pub async fn update<Executing: Executor>(
        self,
        executor: &Executing,
        assign: impl FnOnce(Modeled::Columns<Assigning>) -> Assigned<Modeled, Assigning>,
    ) -> Result<u64, ActiveRecordError> {
        bulk_update(
            self.narrowed.clause,
            assign(<Modeled::Columns<Assigning> as FieldSet>::FIELDS),
            Modeled::TABLE,
            executor,
        )
        .await
    }
}

impl<Modeled: Record, Continued: ScanOrder<Modeled>> Ranged<Modeled, Continued> {
    #[must_use]
    pub fn ascending(self) -> Scan<Modeled> {
        self.scanned(Direction::Ascending)
    }

    #[must_use]
    pub fn descending(self) -> Scan<Modeled> {
        self.scanned(Direction::Descending)
    }

    fn scanned(self, direction: Direction) -> Scan<Modeled> {
        Scan::new(
            Selection::Matching(self.narrowed.clause),
            ordering_columns(Modeled::TABLE, self.head, Continued::REST),
            direction,
        )
    }
}
