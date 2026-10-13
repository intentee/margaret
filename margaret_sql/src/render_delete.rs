use crate::delete::Delete;
use crate::statement::Statement;
use crate::statement_writer::StatementWriter;

#[must_use]
pub fn render_delete(delete: &Delete) -> Statement {
    StatementWriter::new().delete(delete)
}

#[cfg(test)]
mod tests {
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_delete;
    use crate::comparison::Comparison;
    use crate::condition::Condition;
    use crate::delete::Delete;
    use crate::expression::Expression;
    use crate::returning::Returning;
    use crate::sql_parameter::SqlParameter;
    use crate::table_alias::TableAlias;
    use crate::table_source::TableSource;

    const TARGET: TableAlias = TableAlias { position: 0 };

    #[test]
    fn renders_a_take_that_returns_the_removed_row() {
        let statement = render_delete(&Delete {
            condition: Condition::Compare {
                comparison: Comparison::Equal,
                left: Expression::Column {
                    alias: TARGET,
                    column: "id",
                },
                right: Expression::Parameter(SqlParameter::new(1_i64)),
            },
            returning: Returning::Columns(vec![Expression::Column {
                alias: TARGET,
                column: "state",
            }]),
            target: TableSource {
                alias: TARGET,
                namespace: TableNamespace::Framework,
                table: "pending_authorizations",
            },
        });

        assert_eq!(
            statement.text,
            "DELETE FROM \"margaret\".\"pending_authorizations\" AS \"t0\" WHERE \"t0\".\"id\" = $1 \
             RETURNING \"t0\".\"state\""
        );
        assert_eq!(statement.parameters.len(), 1);
    }
}
