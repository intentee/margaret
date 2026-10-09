use std::marker::PhantomData;

use futures_util::Stream;
use futures_util::TryStreamExt as _;
use futures_util::stream::iter;
use futures_util::stream::try_unfold;

use margaret_database::executor::Executor;
use margaret_sql::direction::Direction;
use margaret_sql::maximum_limit::MAXIMUM_LIMIT;

use crate::active_record_error::ActiveRecordError;
use crate::bounded::Bounded;
use crate::next_page::NextPage;
use crate::page::Page;
use crate::record::Record;
use crate::scan_order::ScanOrder;
use crate::selection::Selection;

enum Streaming<Modeled, Ordering, const BATCH: usize> {
    Finished,
    Pending(Bounded<Modeled, Ordering, BATCH>),
}

pub struct Scan<Modeled, Ordering> {
    pub(crate) direction: Direction,
    ordering: PhantomData<fn(Modeled) -> Ordering>,
    pub(crate) selection: Selection,
}

impl<Modeled: Record, Ordering: ScanOrder<Modeled>> Scan<Modeled, Ordering> {
    pub(crate) fn new(selection: Selection, direction: Direction) -> Self {
        Self {
            direction,
            ordering: PhantomData,
            selection,
        }
    }

    #[must_use]
    pub fn limit<const LIMIT: usize>(self) -> Bounded<Modeled, Ordering, LIMIT> {
        const {
            assert!(LIMIT > 0 && LIMIT as u64 <= MAXIMUM_LIMIT);
        }

        Bounded::new(self)
    }

    pub fn stream<const BATCH: usize, Executing: Executor>(
        self,
        executor: &Executing,
    ) -> impl Stream<Item = Result<Modeled, ActiveRecordError>> + Send + '_ {
        try_unfold(
            Streaming::Pending(self.limit::<BATCH>()),
            move |streaming| async move {
                match streaming {
                    Streaming::Finished => Ok(None),
                    Streaming::Pending(bounded) => {
                        bounded
                            .clone()
                            .fetch(executor)
                            .await
                            .map(|Page { next, records }| {
                                Some((
                                    records,
                                    match next {
                                        NextPage::Continues(cursor) => {
                                            Streaming::Pending(bounded.resume(cursor))
                                        }
                                        NextPage::Exhausted => Streaming::Finished,
                                    },
                                ))
                            })
                    }
                }
            },
        )
        .map_ok(|records| iter(records.into_iter().map(Ok)))
        .try_flatten()
    }
}

impl<Modeled, Ordering> Clone for Scan<Modeled, Ordering> {
    fn clone(&self) -> Self {
        Self {
            direction: self.direction,
            ordering: PhantomData,
            selection: self.selection.clone(),
        }
    }
}
