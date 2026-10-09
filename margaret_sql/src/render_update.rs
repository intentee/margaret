use crate::statement::Statement;
use crate::statement_writer::StatementWriter;
use crate::update::Update;

#[must_use]
pub fn render_update(update: &Update) -> Statement {
    StatementWriter::new().update(update)
}

#[cfg(test)]
mod tests {
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_update;
    use crate::assignment::Assignment;
    use crate::assignments::Assignments;
    use crate::comparison::Comparison;
    use crate::condition::Condition;
    use crate::expression::Expression;
    use crate::returning::Returning;
    use crate::sql_parameter::SqlParameter;
    use crate::table_alias::TableAlias;
    use crate::table_source::TableSource;
    use crate::update::Update;

    const TARGET: TableAlias = TableAlias { position: 0 };

    fn compared(column: &'static str, comparison: Comparison, value: i64) -> Condition {
        Condition::Compare {
            comparison,
            left: Expression::Column {
                alias: TARGET,
                column,
            },
            right: Expression::Parameter(SqlParameter::new(value)),
        }
    }

    #[test]
    fn renders_a_compare_and_swap_that_returns_the_row() {
        let statement = render_update(&Update {
            assignments: Assignments {
                first: Assignment {
                    columns: vec!["generation"],
                    values: vec![Expression::Parameter(SqlParameter::new(2_i64))],
                },
                rest: vec![Assignment {
                    columns: vec!["document"],
                    values: vec![Expression::Parameter(SqlParameter::new("{}".to_string()))],
                }],
            },
            condition: Condition::And {
                left: Box::new(compared("name", Comparison::Equal, 1)),
                right: Box::new(compared("generation", Comparison::Equal, 1)),
            },
            returning: Returning::Columns(vec![Expression::Column {
                alias: TARGET,
                column: "generation",
            }]),
            target: TableSource {
                alias: TARGET,
                namespace: TableNamespace::Framework,
                table: "signing_key_sets",
            },
        });

        assert_eq!(
            statement.text,
            "UPDATE \"margaret\".\"signing_key_sets\" AS \"t0\" SET \"generation\" = $1, \"document\" = $2 \
             WHERE (\"t0\".\"name\" = $3 AND \"t0\".\"generation\" = $4) RETURNING \"t0\".\"generation\""
        );
        assert_eq!(statement.parameters.len(), 4);
    }

    #[test]
    fn renders_a_bulk_update_along_an_index_range() {
        assert_eq!(
            render_update(&Update {
                assignments: Assignments {
                    first: Assignment {
                        columns: vec!["body"],
                        values: vec![Expression::Parameter(SqlParameter::new(
                            "swept".to_string()
                        ))],
                    },
                    rest: Vec::new(),
                },
                condition: Condition::And {
                    left: Box::new(compared("expires_at", Comparison::GreaterOrEqual, 1)),
                    right: Box::new(compared("expires_at", Comparison::Less, 9)),
                },
                returning: Returning::Nothing,
                target: TableSource {
                    alias: TARGET,
                    namespace: TableNamespace::Application,
                    table: "notes",
                },
            })
            .text,
            "UPDATE \"notes\" AS \"t0\" SET \"body\" = $1 \
             WHERE (\"t0\".\"expires_at\" >= $2 AND \"t0\".\"expires_at\" < $3)"
        );
    }

    #[test]
    fn renders_an_assignment_that_spans_several_columns() {
        assert_eq!(
            render_update(&Update {
                assignments: Assignments {
                    first: Assignment {
                        columns: vec!["article_id", "locale"],
                        values: vec![
                            Expression::Parameter(SqlParameter::new(1_i64)),
                            Expression::Parameter(SqlParameter::new("en".to_string())),
                        ],
                    },
                    rest: Vec::new(),
                },
                condition: compared("id", Comparison::Equal, 3),
                returning: Returning::Nothing,
                target: TableSource {
                    alias: TARGET,
                    namespace: TableNamespace::Application,
                    table: "translation_notes",
                },
            })
            .text,
            "UPDATE \"translation_notes\" AS \"t0\" SET (\"article_id\", \"locale\") = ($1, $2) \
             WHERE \"t0\".\"id\" = $3"
        );
    }
}
