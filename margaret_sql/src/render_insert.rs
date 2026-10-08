use crate::insert::Insert;
use crate::statement::Statement;
use crate::statement_writer::StatementWriter;

#[must_use]
pub fn render_insert(insert: &Insert) -> Statement {
    StatementWriter::new().insert(insert)
}

#[cfg(test)]
mod tests {
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_insert;
    use crate::assignment::Assignment;
    use crate::assignments::Assignments;
    use crate::comparison::Comparison;
    use crate::condition::Condition;
    use crate::conflict_action::ConflictAction;
    use crate::conflict_filter::ConflictFilter;
    use crate::exists_probe::ExistsProbe;
    use crate::expression::Expression;
    use crate::insert::Insert;
    use crate::insert_source::InsertSource;
    use crate::insert_value::InsertValue;
    use crate::returning::Returning;
    use crate::sql_parameter::SqlParameter;
    use crate::table_alias::TableAlias;
    use crate::table_source::TableSource;

    const TARGET: TableAlias = TableAlias { position: 0 };

    fn target(table: &'static str) -> TableSource {
        TableSource {
            alias: TARGET,
            namespace: TableNamespace::Framework,
            table,
        }
    }

    fn value(column: &'static str, value: i64) -> InsertValue {
        InsertValue {
            column,
            value: Expression::Parameter(SqlParameter::new(value)),
        }
    }

    #[test]
    fn renders_an_insert_that_returns_the_row() {
        let statement = render_insert(&Insert {
            conflict: ConflictAction::Raise,
            returning: Returning::Columns(vec![
                Expression::Column {
                    alias: TARGET,
                    column: "id",
                },
                Expression::Column {
                    alias: TARGET,
                    column: "posted_at",
                },
            ]),
            source: InsertSource::Values,
            target: target("messages"),
            values: vec![value("posted_at", 3)],
        });

        assert_eq!(
            statement.text,
            "INSERT INTO \"margaret\".\"messages\" AS \"t0\" (\"posted_at\") VALUES ($1) \
             RETURNING \"t0\".\"id\", \"t0\".\"posted_at\""
        );
        assert_eq!(statement.parameters.len(), 1);
    }

    #[test]
    fn renders_a_row_of_defaults() {
        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Raise,
                returning: Returning::Nothing,
                source: InsertSource::Values,
                target: target("tokens"),
                values: Vec::new(),
            })
            .text,
            "INSERT INTO \"margaret\".\"tokens\" AS \"t0\" DEFAULT VALUES"
        );
    }

    #[test]
    fn renders_an_insert_guarded_by_an_existence_probe() {
        let authors = TableAlias { position: 1 };

        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Raise,
                returning: Returning::Nothing,
                source: InsertSource::Guarded(Condition::Exists(ExistsProbe {
                    condition: Box::new(Condition::Compare {
                        comparison: Comparison::Equal,
                        left: Expression::Column {
                            alias: authors,
                            column: "id",
                        },
                        right: Expression::Parameter(SqlParameter::new(4_i64)),
                    }),
                    source: TableSource {
                        alias: authors,
                        namespace: TableNamespace::Application,
                        table: "authors",
                    },
                })),
                target: target("articles"),
                values: vec![value("author_id", 4), value("score", 2)],
            })
            .text,
            "INSERT INTO \"margaret\".\"articles\" AS \"t0\" (\"author_id\", \"score\") SELECT $1, $2 \
             WHERE EXISTS (SELECT 1 FROM \"authors\" AS \"t1\" WHERE \"t1\".\"id\" = $3)"
        );
    }

    #[test]
    fn renders_a_guarded_row_of_defaults() {
        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Raise,
                returning: Returning::Nothing,
                source: InsertSource::Guarded(Condition::IsNull(vec![Expression::Parameter(
                    SqlParameter::new(Option::<i64>::None)
                )])),
                target: target("tokens"),
                values: Vec::new(),
            })
            .text,
            "INSERT INTO \"margaret\".\"tokens\" AS \"t0\" SELECT WHERE ($1) IS NULL"
        );
    }

    #[test]
    fn renders_an_insert_that_ignores_a_conflict() {
        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Ignore {
                    target: vec!["name"],
                },
                returning: Returning::Nothing,
                source: InsertSource::Values,
                target: target("signing_key_sets"),
                values: vec![value("generation", 1)],
            })
            .text,
            "INSERT INTO \"margaret\".\"signing_key_sets\" AS \"t0\" (\"generation\") VALUES ($1) \
             ON CONFLICT (\"name\") DO NOTHING"
        );
    }

    #[test]
    fn renders_an_upsert_that_keeps_the_greater_expiry() {
        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Update {
                    assignments: Assignments {
                        first: Assignment {
                            columns: vec!["expires_at"],
                            values: vec![Expression::Greatest {
                                first: Box::new(Expression::Column {
                                    alias: TARGET,
                                    column: "expires_at",
                                }),
                                second: Box::new(Expression::Excluded {
                                    column: "expires_at",
                                }),
                            }],
                        },
                        rest: Vec::new(),
                    },
                    filter: ConflictFilter::Always,
                    target: vec!["family"],
                },
                returning: Returning::Nothing,
                source: InsertSource::Values,
                target: target("refresh_family_revocations"),
                values: vec![value("family", 1), value("expires_at", 2)],
            })
            .text,
            "INSERT INTO \"margaret\".\"refresh_family_revocations\" AS \"t0\" (\"family\", \"expires_at\") VALUES ($1, $2) \
             ON CONFLICT (\"family\") DO UPDATE SET \"expires_at\" = GREATEST(\"t0\".\"expires_at\", EXCLUDED.\"expires_at\")"
        );
    }

    #[test]
    fn renders_an_upsert_that_only_revives_an_expired_row() {
        assert_eq!(
            render_insert(&Insert {
                conflict: ConflictAction::Update {
                    assignments: Assignments {
                        first: Assignment {
                            columns: vec!["expires_at"],
                            values: vec![Expression::Excluded {
                                column: "expires_at",
                            }],
                        },
                        rest: Vec::new(),
                    },
                    filter: ConflictFilter::When(Condition::Compare {
                        comparison: Comparison::LessOrEqual,
                        left: Expression::Column {
                            alias: TARGET,
                            column: "expires_at",
                        },
                        right: Expression::Parameter(SqlParameter::new(4_i64)),
                    }),
                    target: vec!["client_id", "assertion"],
                },
                returning: Returning::Nothing,
                source: InsertSource::Values,
                target: target("client_assertions"),
                values: vec![value("expires_at", 3)],
            })
            .text,
            "INSERT INTO \"margaret\".\"client_assertions\" AS \"t0\" (\"expires_at\") VALUES ($1) \
             ON CONFLICT (\"client_id\", \"assertion\") DO UPDATE SET \"expires_at\" = EXCLUDED.\"expires_at\" \
             WHERE \"t0\".\"expires_at\" <= $2"
        );
    }
}
