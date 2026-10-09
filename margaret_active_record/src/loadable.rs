use std::future::Future;
use std::future::ready;

use margaret_database::executor::Executor;

use crate::active_record_error::ActiveRecordError;
use crate::join_context::JoinContext;
use crate::joined_source::JoinedSource;
use crate::load_cursor::LoadCursor;
use crate::loaded_rows::LoadedRows;
use crate::nothing_preloaded::NothingPreloaded;
use crate::record::Record;
use crate::select_builder::SelectBuilder;

pub trait Loadable: Sized + Send + Sync + 'static {
    type Preloaded: Send + Sync;
    type Root: Record;

    const JOIN: JoinContext;

    const WIDTH: usize;

    fn preload<Executing: Executor>(
        rows: &LoadedRows<'_>,
        offset: usize,
        executor: &Executing,
    ) -> impl Future<Output = Result<Self::Preloaded, ActiveRecordError>> + Send;

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row does not decode into the loaded value.
    fn read(
        cursor: &mut LoadCursor<'_>,
        preloaded: &mut Self::Preloaded,
    ) -> Result<Self, ActiveRecordError>;

    fn select(builder: &mut SelectBuilder, source: JoinedSource);
}

impl<Loaded: Record> Loadable for Loaded {
    type Preloaded = NothingPreloaded;
    type Root = Loaded;

    const JOIN: JoinContext = JoinContext::Required;

    const WIDTH: usize = Loaded::TABLE.columns.len();

    fn preload<Executing: Executor>(
        _: &LoadedRows<'_>,
        _: usize,
        _: &Executing,
    ) -> impl Future<Output = Result<Self::Preloaded, ActiveRecordError>> + Send {
        ready(Ok(NothingPreloaded))
    }

    fn read(
        cursor: &mut LoadCursor<'_>,
        _: &mut Self::Preloaded,
    ) -> Result<Self, ActiveRecordError> {
        cursor.record()
    }

    fn select(builder: &mut SelectBuilder, source: JoinedSource) {
        builder.record::<Loaded>(source);
    }
}

impl<Loaded: Loadable> Loadable for Option<Loaded> {
    type Preloaded = Loaded::Preloaded;
    type Root = Loaded::Root;

    const JOIN: JoinContext = JoinContext::Optional;

    const WIDTH: usize = Loaded::WIDTH;

    fn preload<Executing: Executor>(
        rows: &LoadedRows<'_>,
        offset: usize,
        executor: &Executing,
    ) -> impl Future<Output = Result<Self::Preloaded, ActiveRecordError>> + Send {
        Loaded::preload(rows, offset, executor)
    }

    fn read(
        cursor: &mut LoadCursor<'_>,
        preloaded: &mut Self::Preloaded,
    ) -> Result<Self, ActiveRecordError> {
        cursor
            .next_columns_are_null(Loaded::WIDTH, Loaded::Root::TABLE)
            .and_then(|null| {
                if null {
                    cursor.skip(Loaded::WIDTH);

                    Ok(None)
                } else {
                    Loaded::read(cursor, preloaded).map(Some)
                }
            })
    }

    fn select(builder: &mut SelectBuilder, source: JoinedSource) {
        Loaded::select(builder, source);
    }
}
