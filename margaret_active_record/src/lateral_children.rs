use std::collections::HashMap;

use futures_util::TryFutureExt as _;

use tokio_postgres::Row;

use margaret_database::executor::Executor;
use margaret_model::column_type::ColumnType;
use margaret_sql::array_parameter::ArrayParameter;
use margaret_sql::comparison::Comparison;
use margaret_sql::condition::Condition;
use margaret_sql::direction::Direction;
use margaret_sql::expression::Expression;
use margaret_sql::from_item::FromItem;
use margaret_sql::join::Join;
use margaret_sql::order_term::OrderTerm;
use margaret_sql::render_select::render_select;
use margaret_sql::select::Select;
use margaret_sql::select_filter::SelectFilter;
use margaret_sql::select_limit::SelectLimit;
use margaret_sql::select_lock::SelectLock;
use margaret_sql::sql_parameter::SqlParameter;
use margaret_sql::table_alias::TableAlias;
use margaret_sql::unnest_source::UnnestSource;

use crate::active_record_error::ActiveRecordError;
use crate::column_presence::ColumnPresence;
use crate::executed_rows::executed_rows;
use crate::field_span::FieldSpan;
use crate::join_context::JoinContext;
use crate::joined_source::JoinedSource;
use crate::loadable::Loadable;
use crate::loaded_records::loaded_records;
use crate::loaded_rows::LoadedRows;
use crate::raw_column::RawColumn;
use crate::record::Record;
use crate::row_column::row_column;
use crate::select_builder::SelectBuilder;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;

const PARENTS: TableAlias = TableAlias { position: 0 };
const CHILDREN: TableAlias = TableAlias { position: 1 };

struct ParentKeys {
    arrays: Vec<Vec<RawColumn>>,
    indexes: Vec<i64>,
}

fn parent_keys<Child: Loadable>(
    rows: &[Row],
    key_offset: usize,
    key: FieldSpan,
) -> Result<ParentKeys, ActiveRecordError> {
    rows.iter().zip(0_i64..).try_fold(
        ParentKeys {
            arrays: (0..key.width).map(|_| Vec::new()).collect(),
            indexes: Vec::new(),
        },
        |mut keys, (row, index)| {
            row_column::<ColumnPresence>(row, key_offset, Child::Root::TABLE).and_then(|presence| {
                match presence {
                    ColumnPresence::Null => Ok(keys),
                    ColumnPresence::Present => (key_offset..key_offset + key.width)
                        .map(|position| row_column::<RawColumn>(row, position, Child::Root::TABLE))
                        .collect::<Result<Vec<RawColumn>, ActiveRecordError>>()
                        .map(|values| {
                            for (array, value) in keys.arrays.iter_mut().zip(values) {
                                array.push(value);
                            }

                            keys.indexes.push(index);
                            keys
                        }),
                }
            })
        },
    )
}

struct ChildQuery {
    lateral: TableAlias,
    select: Select,
}

fn child_query<Child: Loadable>(
    key: FieldSpan,
    order: &[FieldSpan],
    limit: SelectLimit,
) -> ChildQuery {
    let table = Child::Root::TABLE;
    let mut builder = SelectBuilder::new(CHILDREN);

    Child::select(
        &mut builder,
        JoinedSource {
            alias: CHILDREN,
            context: JoinContext::Required,
        },
    );

    let lateral = builder.alias();
    let SelectBuilder { columns, joins, .. } = builder;
    let select = Select {
        columns,
        filter: SelectFilter::Matching(Condition::RowCompare {
            comparison: Comparison::Equal,
            left: table
                .columns
                .iter()
                .skip(key.start)
                .take(key.width)
                .map(|column| Expression::Column {
                    alias: CHILDREN,
                    column: column.name,
                })
                .collect(),
            right: (0..key.width)
                .map(|position| Expression::UnnestColumn {
                    alias: PARENTS,
                    position,
                })
                .collect(),
        }),
        from: FromItem::Table(table_source(table, CHILDREN)),
        joins,
        limit,
        lock: SelectLock::Unlocked,
        ordering: order
            .iter()
            .flat_map(|span| table.columns.iter().skip(span.start).take(span.width))
            .map(|column| OrderTerm {
                direction: Direction::Ascending,
                expression: Expression::Column {
                    alias: CHILDREN,
                    column: column.name,
                },
            })
            .collect(),
    };

    ChildQuery { lateral, select }
}

fn lateral_select<Child: Loadable>(
    keys: ParentKeys,
    key: FieldSpan,
    order: &[FieldSpan],
    limit: SelectLimit,
) -> Select {
    let ChildQuery { lateral, select } = child_query::<Child>(key, order, limit);
    let key_types = Child::Root::TABLE
        .columns
        .iter()
        .skip(key.start)
        .take(key.width)
        .map(|column| column.column_type);
    let mut arrays: Vec<ArrayParameter> = key_types
        .zip(keys.arrays)
        .map(|(element_type, values)| ArrayParameter {
            element_type,
            values: SqlParameter::new(values),
        })
        .collect();

    arrays.push(ArrayParameter {
        element_type: ColumnType::BigInt,
        values: SqlParameter::new(keys.indexes),
    });

    let parent_index = Expression::UnnestColumn {
        alias: PARENTS,
        position: key.width,
    };

    Select {
        columns: [parent_index.clone()]
            .into_iter()
            .chain(
                (0..Child::WIDTH).map(|position| Expression::SubqueryColumn {
                    alias: lateral,
                    position,
                }),
            )
            .collect(),
        filter: SelectFilter::Everything,
        from: FromItem::Unnest(UnnestSource {
            alias: PARENTS,
            arrays,
        }),
        joins: vec![Join::Lateral {
            alias: lateral,
            select: Box::new(select),
        }],
        limit: SelectLimit::Unlimited,
        lock: SelectLock::Unlocked,
        ordering: [parent_index]
            .into_iter()
            .chain(
                order
                    .iter()
                    .flat_map(|span| span.start..span.start + span.width)
                    .map(|position| Expression::SubqueryColumn {
                        alias: lateral,
                        position,
                    }),
            )
            .map(|expression| OrderTerm {
                direction: Direction::Ascending,
                expression,
            })
            .collect(),
    }
}

pub(crate) struct LateralChildren<'relation> {
    pub(crate) key: FieldSpan,
    pub(crate) limit: SelectLimit,
    pub(crate) order: &'relation [FieldSpan],
}

impl LateralChildren<'_> {
    pub(crate) async fn grouped<Child: Loadable, Executing: Executor>(
        &self,
        LoadedRows { rows }: &LoadedRows<'_>,
        key_offset: usize,
        executor: &Executing,
    ) -> Result<HashMap<i64, Vec<Child>>, ActiveRecordError> {
        let table = Child::Root::TABLE;
        let keys = parent_keys::<Child>(rows, key_offset, self.key);

        match keys {
            Ok(keys) if keys.indexes.is_empty() => Ok(HashMap::new()),
            keys => {
                let statement = keys.map(|keys| {
                    render_select(&lateral_select::<Child>(
                        keys, self.key, self.order, self.limit,
                    ))
                });

                executed_rows(statement, StatementKind::Select, table, executor)
                    .and_then(|child_rows| async move {
                        let parents = child_rows
                            .iter()
                            .map(|row| row_column::<i64>(row, 0, table))
                            .collect::<Result<Vec<i64>, ActiveRecordError>>();

                        loaded_records::<Child, _>(&child_rows, 1, executor)
                            .await
                            .and_then(|children| {
                                parents.map(|parents| {
                                    let mut groups: HashMap<i64, Vec<Child>> = HashMap::new();

                                    for (parent, child) in parents.into_iter().zip(children) {
                                        groups.entry(parent).or_default().push(child);
                                    }

                                    groups
                                })
                            })
                    })
                    .await
            }
        }
    }
}
