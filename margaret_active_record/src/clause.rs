use std::sync::Arc;

use margaret_model::table::Table;
use margaret_sql::comparison::Comparison;
use margaret_sql::condition::Condition;
use margaret_sql::exists_probe::ExistsProbe;
use margaret_sql::expression::Expression;
use margaret_sql::table_alias::TableAlias;

use crate::active_record_error::ActiveRecordError;
use crate::encodable::Encodable;
use crate::encoded_values::encoded_values;
use crate::table_source::table_source;

fn column_expressions(columns: &[&'static str], alias: TableAlias) -> Vec<Expression> {
    columns
        .iter()
        .map(|column| Expression::Column { alias, column })
        .collect()
}

#[derive(Clone)]
pub enum Clause {
    And {
        left: Box<Clause>,
        right: Box<Clause>,
    },
    Compare {
        columns: Vec<&'static str>,
        comparison: Comparison,
        value: Arc<dyn Encodable>,
    },
    Exists {
        clause: Box<Clause>,
        table: &'static Table,
    },
    IsNull {
        columns: Vec<&'static str>,
    },
    Not(Box<Clause>),
}

impl Clause {
    pub(crate) fn and(self, other: Self) -> Self {
        Self::And {
            left: Box::new(self),
            right: Box::new(other),
        }
    }

    pub(crate) fn condition(
        &self,
        alias: TableAlias,
        table: &'static Table,
    ) -> Result<Condition, ActiveRecordError> {
        match self {
            Self::And { left, right } => left.condition(alias, table).and_then(|left| {
                right.condition(alias, table).map(|right| Condition::And {
                    left: Box::new(left),
                    right: Box::new(right),
                })
            }),
            Self::Compare {
                columns,
                comparison,
                value,
            } => encoded_values(value.as_ref(), table).map(|right| Condition::RowCompare {
                comparison: *comparison,
                left: column_expressions(columns, alias),
                right,
            }),
            Self::Exists {
                clause,
                table: probed,
            } => {
                let probe_alias = TableAlias {
                    position: alias.position + 1,
                };

                clause.condition(probe_alias, probed).map(|condition| {
                    Condition::Exists(ExistsProbe {
                        condition: Box::new(condition),
                        source: table_source(probed, probe_alias),
                    })
                })
            }
            Self::IsNull { columns } => Ok(Condition::IsNull(column_expressions(columns, alias))),
            Self::Not(negated) => negated
                .condition(alias, table)
                .map(|condition| Condition::Not(Box::new(condition))),
        }
    }
}
