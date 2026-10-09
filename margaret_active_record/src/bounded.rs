use std::future::ready;
use std::sync::Arc;

use futures_util::TryFutureExt as _;

use margaret_database::executor::Executor;
use margaret_sql::expression::Expression;
use margaret_sql::from_item::FromItem;
use margaret_sql::join::Join;
use margaret_sql::order_term::OrderTerm;
use margaret_sql::render_select::render_select;
use margaret_sql::select::Select;
use margaret_sql::select_limit::SelectLimit;
use margaret_sql::select_lock::SelectLock;
use margaret_sql::statement::Statement;

use crate::active_record_error::ActiveRecordError;
use crate::base_alias::BASE_ALIAS;
use crate::clause::Clause;
use crate::executed_rows::executed_rows;
use crate::join_context::JoinContext;
use crate::joined_source::JoinedSource;
use crate::keyset_position::KeysetPosition;
use crate::loaded_records::loaded_records;
use crate::ordering_columns::ordering_columns;
use crate::page::Page;
use crate::page_cursor::PageCursor;
use crate::page_rows::next_page;
use crate::read_record::read_record;
use crate::record::Record;
use crate::record_columns::record_columns;
use crate::scan::Scan;
use crate::scan_direction::ScanDirection;
use crate::scan_order::ScanOrder;
use crate::select_builder::SelectBuilder;
use crate::select_filter::select_filter;
use crate::shape::Shape;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;

pub struct Bounded<Modeled, Ordering, Toward, const LIMIT: usize> {
    position: KeysetPosition<Modeled, Ordering, Toward>,
    scan: Scan<Modeled, Ordering, Toward>,
}

impl<Modeled, Ordering, Toward, const LIMIT: usize> Bounded<Modeled, Ordering, Toward, LIMIT>
where
    Modeled: Record,
    Ordering: ScanOrder<Modeled>,
    Toward: ScanDirection,
{
    const FETCHED_ROWS: u64 = LIMIT as u64 + 1;

    pub(crate) fn new(scan: Scan<Modeled, Ordering, Toward>) -> Self {
        Self {
            position: KeysetPosition::Start,
            scan,
        }
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the page cannot be read.
    pub async fn fetch<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Page<Modeled, Modeled, Ordering, Toward>, ActiveRecordError> {
        let order = ordering_columns(Modeled::TABLE, Ordering::SPANS);
        let cursor_width = order.len();
        let statement = self.selected(
            order,
            record_columns(Modeled::TABLE, BASE_ALIAS),
            Vec::new(),
        );

        executed_rows(statement, StatementKind::Select, Modeled::TABLE, executor)
            .await
            .and_then(|rows| {
                next_page(
                    &rows,
                    LIMIT,
                    Modeled::TABLE.columns.len(),
                    cursor_width,
                    Modeled::TABLE,
                )
                .and_then(|next| {
                    rows.iter()
                        .take(LIMIT)
                        .map(read_record)
                        .collect::<Result<Vec<Modeled>, ActiveRecordError>>()
                        .map(|records| Page { next, records })
                })
            })
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the page or its relations cannot be read.
    pub async fn load<Loaded: Shape<Root = Modeled>, Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Page<Loaded, Modeled, Ordering, Toward>, ActiveRecordError> {
        let mut builder = SelectBuilder::new(BASE_ALIAS);

        Loaded::select(
            &mut builder,
            JoinedSource {
                alias: BASE_ALIAS,
                context: JoinContext::Required,
            },
        );

        let SelectBuilder { columns, joins, .. } = builder;
        let order = ordering_columns(Modeled::TABLE, Ordering::SPANS);
        let cursor_width = order.len();
        let statement = self.selected(order, columns, joins);

        executed_rows(statement, StatementKind::Select, Modeled::TABLE, executor)
            .and_then(|rows| async move {
                ready(next_page(
                    &rows,
                    LIMIT,
                    Loaded::WIDTH,
                    cursor_width,
                    Modeled::TABLE,
                ))
                .and_then(|next| {
                    loaded_records::<Loaded, _>(&rows[..rows.len().min(LIMIT)], 0, executor)
                        .map_ok(|records| Page { next, records })
                })
                .await
            })
            .await
    }

    #[must_use]
    pub fn resume(self, cursor: PageCursor<Modeled, Ordering, Toward>) -> Self {
        Self {
            position: KeysetPosition::From(cursor),
            scan: self.scan,
        }
    }

    fn selected(
        self,
        order: Vec<&'static str>,
        columns: Vec<Expression>,
        joins: Vec<Join>,
    ) -> Result<Statement, ActiveRecordError> {
        let Scan { selection, .. } = self.scan;
        let ordering: Vec<Expression> = order
            .iter()
            .map(|column| Expression::Column {
                alias: BASE_ALIAS,
                column,
            })
            .collect();
        let positioned = match self.position {
            KeysetPosition::From(cursor) => selection.and(Clause::Compare {
                columns: order,
                comparison: Toward::CURSOR_COMPARISON,
                value: Arc::new(cursor),
            }),
            KeysetPosition::Start => selection,
        };

        select_filter(positioned, Modeled::TABLE).map(|filter| {
            render_select(&Select {
                columns: columns
                    .into_iter()
                    .chain(ordering.iter().cloned())
                    .collect(),
                filter,
                from: FromItem::Table(table_source(Modeled::TABLE, BASE_ALIAS)),
                joins,
                limit: SelectLimit::Rows(Self::FETCHED_ROWS),
                lock: SelectLock::Unlocked,
                ordering: ordering
                    .into_iter()
                    .map(|expression| OrderTerm {
                        direction: Toward::DIRECTION,
                        expression,
                    })
                    .collect(),
            })
        })
    }
}

impl<Modeled, Ordering, Toward, const LIMIT: usize> Clone
    for Bounded<Modeled, Ordering, Toward, LIMIT>
{
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            scan: self.scan.clone(),
        }
    }
}
