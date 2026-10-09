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
use crate::record::Record;
use crate::scan::Scan;
use crate::scan_order::ScanOrder;
use crate::selection::Selection;

pub struct Ranged<Modeled, Ordering> {
    narrowed: Narrowed<Modeled>,
    ordering: PhantomData<fn() -> Ordering>,
}

impl<Modeled: Record, Ordering> Ranged<Modeled, Ordering> {
    pub(crate) fn new(narrowed: Narrowed<Modeled>) -> Self {
        Self {
            narrowed,
            ordering: PhantomData,
        }
    }
}

impl<Modeled: Model, Ordering> Ranged<Modeled, Ordering> {
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

impl<Modeled: Assignable, Ordering> Ranged<Modeled, Ordering> {
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

impl<Modeled: Record, Ordering: ScanOrder<Modeled>> Ranged<Modeled, Ordering> {
    #[must_use]
    pub fn ascending(self) -> Scan<Modeled, Ordering> {
        self.scanned(Direction::Ascending)
    }

    #[must_use]
    pub fn descending(self) -> Scan<Modeled, Ordering> {
        self.scanned(Direction::Descending)
    }

    fn scanned(self, direction: Direction) -> Scan<Modeled, Ordering> {
        Scan::new(Selection::Matching(self.narrowed.clause), direction)
    }
}
