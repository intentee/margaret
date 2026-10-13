use margaret_database::executor::Executor;

use crate::active_record_error::ActiveRecordError;
use crate::assignable::Assignable;
use crate::assigned::Assigned;
use crate::assigning::Assigning;
use crate::bulk_delete::bulk_delete;
use crate::bulk_update::bulk_update;
use crate::continuation::Continuation;
use crate::field_set::FieldSet;
use crate::model::Model;
use crate::narrowed::Narrowed;

pub struct Prefix<Modeled> {
    narrowed: Narrowed<Modeled>,
}

impl<Modeled: Model> Prefix<Modeled> {
    #[must_use]
    pub fn new(narrowed: Narrowed<Modeled>) -> Self {
        Self { narrowed }
    }

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

impl<Modeled: Assignable> Prefix<Modeled> {
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

impl<Modeled: Model> Continuation<Modeled> for Prefix<Modeled> {
    fn continued(narrowed: Narrowed<Modeled>) -> Self {
        Self::new(narrowed)
    }
}
