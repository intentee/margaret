use futures_util::TryFutureExt as _;

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
use margaret_sql::sql_parameter::SqlParameter;
use margaret_sql::statement::Statement;
use margaret_sql::table_alias::TableAlias;
use margaret_sql::unnest_source::UnnestSource;

use crate::active_record_error::ActiveRecordError;
use crate::dyn_parameter::DynParameter;
use crate::executed_rows::executed_rows;
use crate::join_context::JoinContext;
use crate::joined_source::JoinedSource;
use crate::loadable::Loadable;
use crate::loaded_records::loaded_records;
use crate::parameters::Parameters;
use crate::primary_key_columns::primary_key_columns;
use crate::record::Record;
use crate::select_builder::SelectBuilder;
use crate::statement_kind::StatementKind;
use crate::table_source::table_source;
use crate::value::Value;

const KEYS: TableAlias = TableAlias { position: 0 };
const ROOTS: TableAlias = TableAlias { position: 1 };

struct KeyArrays {
    indexes: Vec<i64>,
    keys: Vec<Vec<DynParameter>>,
}

fn key_arrays<Loaded: Loadable>(
    records: &[Loaded::Root],
    width: usize,
) -> Result<KeyArrays, ActiveRecordError> {
    records.iter().zip(0_i64..).try_fold(
        KeyArrays {
            indexes: Vec::new(),
            keys: (0..width).map(|_| Vec::new()).collect(),
        },
        |mut arrays, (record, index)| {
            let mut parameters = Parameters::new(Loaded::Root::TABLE);

            record.primary_key().write(&mut parameters).map(|()| {
                for (column, parameter) in arrays.keys.iter_mut().zip(parameters.into_values()) {
                    column.push(DynParameter { parameter });
                }

                arrays.indexes.push(index);
                arrays
            })
        },
    )
}

fn keyed_select<Loaded: Loadable>(KeyArrays { indexes, keys }: KeyArrays) -> Statement {
    let table = Loaded::Root::TABLE;
    let key_types: Vec<ColumnType> = Loaded::Root::PRIMARY_KEY
        .iter()
        .flat_map(|span| table.columns.iter().skip(span.start).take(span.width))
        .map(|column| column.column_type)
        .collect();
    let width = key_types.len();
    let mut arrays: Vec<ArrayParameter> = key_types
        .into_iter()
        .zip(keys)
        .map(|(element_type, values)| ArrayParameter {
            element_type,
            values: SqlParameter::new(values),
        })
        .collect();

    arrays.push(ArrayParameter {
        element_type: ColumnType::BigInt,
        values: SqlParameter::new(indexes),
    });

    let mut builder = SelectBuilder::new(ROOTS);

    Loaded::select(
        &mut builder,
        JoinedSource {
            alias: ROOTS,
            context: JoinContext::Required,
        },
    );

    let SelectBuilder { columns, joins, .. } = builder;

    render_select(&Select {
        columns,
        filter: SelectFilter::Everything,
        from: FromItem::Unnest(UnnestSource {
            alias: KEYS,
            arrays,
        }),
        joins: [Join::Inner {
            on: Condition::RowCompare {
                comparison: Comparison::Equal,
                left: primary_key_columns::<Loaded::Root>()
                    .into_iter()
                    .map(|column| Expression::Column {
                        alias: ROOTS,
                        column,
                    })
                    .collect(),
                right: (0..width)
                    .map(|position| Expression::UnnestColumn {
                        alias: KEYS,
                        position,
                    })
                    .collect(),
            },
            source: table_source(table, ROOTS),
        }]
        .into_iter()
        .chain(joins)
        .collect(),
        limit: SelectLimit::Unlimited,
        ordering: vec![OrderTerm {
            direction: Direction::Ascending,
            expression: Expression::UnnestColumn {
                alias: KEYS,
                position: width,
            },
        }],
    })
}

pub(crate) async fn attach_shapes<Loaded: Loadable, Executing: Executor>(
    executor: &Executing,
    records: &[Loaded::Root],
) -> Result<Vec<Loaded>, ActiveRecordError> {
    let table = Loaded::Root::TABLE;

    if records.is_empty() {
        return Ok(Vec::new());
    }

    let statement = key_arrays::<Loaded>(records, primary_key_columns::<Loaded::Root>().len())
        .map(keyed_select::<Loaded>);

    executed_rows(statement, StatementKind::Select, table, executor)
        .and_then(|rows| async move {
            if rows.len() < records.len() {
                Err(ActiveRecordError::ParentVanished { table: table.name })
            } else {
                loaded_records::<Loaded, _>(&rows, 0, executor).await
            }
        })
        .await
}
