use futures_util::TryFutureExt as _;
use tokio_postgres::Row;

use margaret_database::executor::Executor;
use margaret_database::transaction::Transaction;
use margaret_sql::delete::Delete;
use margaret_sql::expression::Expression;
use margaret_sql::from_item::FromItem;
use margaret_sql::join::Join;
use margaret_sql::render_delete::render_delete;
use margaret_sql::render_select::render_select;
use margaret_sql::render_update::render_update;
use margaret_sql::returning::Returning;
use margaret_sql::select::Select;
use margaret_sql::select_filter::SelectFilter;
use margaret_sql::select_limit::SelectLimit;
use margaret_sql::select_lock::SelectLock;
use margaret_sql::statement::Statement;
use margaret_sql::update::Update;

use crate::active_record_error::ActiveRecordError;
use crate::assignable::Assignable;
use crate::assigned::Assigned;
use crate::assigning::Assigning;
use crate::base_alias::BASE_ALIAS;
use crate::change::Change;
use crate::clause::Clause;
use crate::continuation::Continuation;
use crate::executed_optional_row::executed_optional_row;
use crate::executed_rows::executed_rows;
use crate::field_set::FieldSet;
use crate::field_span::FieldSpan;
use crate::join_context::JoinContext;
use crate::joined_source::JoinedSource;
use crate::loadable::Loadable;
use crate::loaded_records::loaded_records;
use crate::lookup::Lookup;
use crate::model::Model;
use crate::narrowed::Narrowed;
use crate::predicate::Predicate;
use crate::read_record::read_record;
use crate::record::Record;
use crate::record_columns::record_columns;
use crate::removal::Removal;
use crate::row_guard::RowGuard;
use crate::scan_order::ScanOrder;
use crate::select_builder::SelectBuilder;
use crate::shape::Shape;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;
use crate::unguarded::Unguarded;

fn change_of<Modeled: Record>(row: Option<Row>) -> Result<Change<Modeled>, ActiveRecordError> {
    match row {
        Some(row) => read_record(&row).map(Change::Changed),
        None => Ok(Change::Unmatched),
    }
}

pub struct Unique<Modeled, Guard> {
    guard: Guard,
    narrowed: Narrowed<Modeled>,
}

impl<Modeled: Model> Unique<Modeled, Unguarded> {
    #[must_use]
    pub fn when(
        self,
        guard: impl FnOnce(Modeled::Conditions) -> Predicate<Modeled>,
    ) -> Unique<Modeled, Predicate<Modeled>> {
        Unique {
            guard: guard(<Modeled::Conditions as FieldSet>::FIELDS),
            narrowed: self.narrowed,
        }
    }
}

impl<Modeled: Model, Guard: RowGuard> Unique<Modeled, Guard> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be deleted.
    pub async fn delete<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Removal<Modeled>, ActiveRecordError> {
        let statement = self
            .clause()
            .condition(BASE_ALIAS, Modeled::TABLE)
            .map(|condition| {
                render_delete(&Delete {
                    condition,
                    returning: Returning::Columns(record_columns(Modeled::TABLE, BASE_ALIAS)),
                    target: table_source(Modeled::TABLE, BASE_ALIAS),
                })
            });

        executed_optional_row(statement, StatementKind::Delete, Modeled::TABLE, executor)
            .await
            .and_then(|row| match row {
                Some(row) => read_record(&row).map(Removal::Removed),
                None => Ok(Removal::Missing),
            })
    }

    #[must_use]
    pub fn exists<Row>(self) -> Predicate<Row> {
        Predicate::new(Clause::Exists {
            clause: Box::new(self.clause()),
            table: Modeled::TABLE,
        })
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be read.
    pub async fn find<Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Lookup<Modeled>, ActiveRecordError> {
        self.found(SelectLock::Unlocked, executor).await
    }

    /// Finds the row and holds it against deletion and key changes until the transaction ends.
    ///
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be read.
    pub async fn find_key_shared(
        self,
        transaction: &Transaction<'_>,
    ) -> Result<Lookup<Modeled>, ActiveRecordError> {
        self.found(SelectLock::KeyShare, transaction).await
    }

    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row or its relations cannot be read.
    pub async fn load<Loaded: Shape<Root = Modeled>, Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Lookup<Loaded>, ActiveRecordError> {
        self.loaded(executor).await
    }

    pub(crate) async fn loaded<Loaded: Loadable<Root = Modeled>, Executing: Executor>(
        self,
        executor: &Executing,
    ) -> Result<Lookup<Loaded>, ActiveRecordError> {
        let mut builder = SelectBuilder::new(BASE_ALIAS);

        Loaded::select(
            &mut builder,
            JoinedSource {
                alias: BASE_ALIAS,
                context: JoinContext::Required,
            },
        );

        let SelectBuilder { columns, joins, .. } = builder;
        let statement = self.selected(columns, joins, SelectLock::Unlocked);

        executed_rows(statement, StatementKind::Select, Modeled::TABLE, executor)
            .and_then(|rows| async move { loaded_records::<Loaded, _>(&rows, 0, executor).await })
            .await
            .map(|loaded| match loaded.into_iter().next() {
                Some(found) => Lookup::Found(found),
                None => Lookup::Missing,
            })
    }

    fn clause(self) -> Clause {
        self.guard.guarded(self.narrowed.clause)
    }

    async fn found<Executing: Executor>(
        self,
        lock: SelectLock,
        executor: &Executing,
    ) -> Result<Lookup<Modeled>, ActiveRecordError> {
        let statement = self.selected(record_columns(Modeled::TABLE, BASE_ALIAS), Vec::new(), lock);

        executed_optional_row(statement, StatementKind::Select, Modeled::TABLE, executor)
            .await
            .and_then(|row| match row {
                Some(row) => read_record(&row).map(Lookup::Found),
                None => Ok(Lookup::Missing),
            })
    }

    fn selected(
        self,
        columns: Vec<Expression>,
        joins: Vec<Join>,
        lock: SelectLock,
    ) -> Result<Statement, ActiveRecordError> {
        self.clause()
            .condition(BASE_ALIAS, Modeled::TABLE)
            .map(|condition| {
                render_select(&Select {
                    columns,
                    filter: SelectFilter::Matching(condition),
                    from: FromItem::Table(table_source(Modeled::TABLE, BASE_ALIAS)),
                    joins,
                    limit: SelectLimit::Unlimited,
                    lock,
                    ordering: Vec::new(),
                })
            })
    }
}

impl<Modeled: Assignable, Guard: RowGuard> Unique<Modeled, Guard> {
    /// # Errors
    ///
    /// Returns `ActiveRecordError` when the row cannot be updated.
    pub async fn update<Executing: Executor>(
        self,
        executor: &Executing,
        assign: impl FnOnce(Modeled::Columns<Assigning>) -> Assigned<Modeled, Assigning>,
    ) -> Result<Change<Modeled>, ActiveRecordError> {
        let clause = self.clause();
        let statement = assign(<Modeled::Columns<Assigning> as FieldSet>::FIELDS)
            .assignments(Modeled::TABLE)
            .and_then(|assignments| {
                clause
                    .condition(BASE_ALIAS, Modeled::TABLE)
                    .map(|condition| {
                        render_update(&Update {
                            assignments,
                            condition,
                            returning: Returning::Columns(record_columns(
                                Modeled::TABLE,
                                BASE_ALIAS,
                            )),
                            target: table_source(Modeled::TABLE, BASE_ALIAS),
                        })
                    })
            });

        executed_optional_row(statement, StatementKind::Update, Modeled::TABLE, executor)
            .await
            .and_then(change_of)
    }
}

impl<Modeled: Record> Continuation<Modeled> for Unique<Modeled, Unguarded> {
    fn continued(narrowed: Narrowed<Modeled>) -> Self {
        Self {
            guard: Unguarded,
            narrowed,
        }
    }
}

impl<Modeled: Record> ScanOrder<Modeled> for Unique<Modeled, Unguarded> {
    const REST: &'static [FieldSpan] = &[];
}
