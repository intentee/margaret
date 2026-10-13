use tokio_postgres::Row;

use margaret_database::executor::Executor;

use crate::active_record_error::ActiveRecordError;
use crate::load_cursor::LoadCursor;
use crate::loadable::Loadable;
use crate::loaded_rows::LoadedRows;

pub(crate) async fn loaded_records<Loaded: Loadable, Executing: Executor>(
    rows: &[Row],
    offset: usize,
    executor: &Executing,
) -> Result<Vec<Loaded>, ActiveRecordError> {
    let mut preloaded = Loaded::preload(&LoadedRows { rows }, offset, executor).await?;

    rows.iter()
        .zip(0_i64..)
        .map(|(row, index)| Loaded::read(&mut LoadCursor::new(row, offset, index), &mut preloaded))
        .collect()
}
