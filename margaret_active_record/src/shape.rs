use std::future::Future;

use margaret_database::executor::Executor;

use crate::active_record_error::ActiveRecordError;
use crate::attach_shapes::attach_shapes;
use crate::loadable::Loadable;

pub trait Shape: Loadable {
    fn attach<Executing: Executor>(
        executor: &Executing,
        records: &[Self::Root],
    ) -> impl Future<Output = Result<Vec<Self>, ActiveRecordError>> + Send {
        attach_shapes(executor, records)
    }
}
