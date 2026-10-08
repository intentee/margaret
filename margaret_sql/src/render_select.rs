use crate::select::Select;
use crate::statement::Statement;
use crate::statement_writer::StatementWriter;

#[must_use]
pub fn render_select(select: &Select) -> Statement {
    StatementWriter::new().select(select)
}

#[cfg(test)]
mod tests {
    use margaret_model::column_type::ColumnType;
    use margaret_sql_identifier::table_namespace::TableNamespace;

    use super::render_select;
    use crate::array_parameter::ArrayParameter;
    use crate::comparison::Comparison;
    use crate::condition::Condition;
    use crate::direction::Direction;
    use crate::exists_probe::ExistsProbe;
    use crate::expression::Expression;
    use crate::from_item::FromItem;
    use crate::join::Join;
    use crate::order_term::OrderTerm;
    use crate::select::Select;
    use crate::select_filter::SelectFilter;
    use crate::select_limit::SelectLimit;
    use crate::sql_parameter::SqlParameter;
    use crate::table_alias::TableAlias;
    use crate::table_source::TableSource;
    use crate::unnest_source::UnnestSource;

    const BASE: TableAlias = TableAlias { position: 0 };
    const JOINED: TableAlias = TableAlias { position: 1 };

    fn column(alias: TableAlias, column: &'static str) -> Expression {
        Expression::Column { alias, column }
    }

    fn articles() -> TableSource {
        TableSource {
            alias: BASE,
            namespace: TableNamespace::Application,
            table: "articles",
        }
    }

    #[test]
    fn renders_an_unfiltered_unbounded_select() {
        let statement = render_select(&Select {
            columns: vec![column(BASE, "id")],
            filter: SelectFilter::Everything,
            from: FromItem::Table(articles()),
            joins: Vec::new(),
            limit: SelectLimit::Unlimited,
            ordering: Vec::new(),
        });

        assert_eq!(
            statement.text,
            "SELECT \"t0\".\"id\" AS \"c0\" FROM \"articles\" AS \"t0\""
        );
        assert!(statement.parameters.is_empty());
    }

    #[test]
    fn renders_a_keyset_page_along_an_index() {
        let statement = render_select(&Select {
            columns: vec![column(BASE, "id"), column(BASE, "posted_at")],
            filter: SelectFilter::Matching(Condition::RowCompare {
                comparison: Comparison::Less,
                left: vec![column(BASE, "posted_at"), column(BASE, "id")],
                right: vec![
                    Expression::Parameter(SqlParameter::new(5_i64)),
                    Expression::Parameter(SqlParameter::new(7_i64)),
                ],
            }),
            from: FromItem::Table(articles()),
            joins: Vec::new(),
            limit: SelectLimit::Rows(21),
            ordering: vec![
                OrderTerm {
                    direction: Direction::Descending,
                    expression: column(BASE, "posted_at"),
                },
                OrderTerm {
                    direction: Direction::Descending,
                    expression: column(BASE, "id"),
                },
            ],
        });

        assert_eq!(
            statement.text,
            "SELECT \"t0\".\"id\" AS \"c0\", \"t0\".\"posted_at\" AS \"c1\" FROM \"articles\" AS \"t0\" \
             WHERE (\"t0\".\"posted_at\", \"t0\".\"id\") < ($1, $2) \
             ORDER BY \"t0\".\"posted_at\" DESC, \"t0\".\"id\" DESC LIMIT 21"
        );
        assert_eq!(statement.parameters.len(), 2);
    }

    #[test]
    fn renders_the_belongs_to_joins_of_a_unique_lookup() {
        let statement = render_select(&Select {
            columns: vec![column(BASE, "id"), column(JOINED, "name")],
            filter: SelectFilter::Matching(Condition::Compare {
                comparison: Comparison::Equal,
                left: column(BASE, "id"),
                right: Expression::Parameter(SqlParameter::new(3_i64)),
            }),
            from: FromItem::Table(articles()),
            joins: vec![
                Join::Inner {
                    on: Condition::Compare {
                        comparison: Comparison::Equal,
                        left: column(JOINED, "id"),
                        right: column(BASE, "author_id"),
                    },
                    source: TableSource {
                        alias: JOINED,
                        namespace: TableNamespace::Application,
                        table: "authors",
                    },
                },
                Join::Left {
                    on: Condition::Compare {
                        comparison: Comparison::Equal,
                        left: column(TableAlias { position: 2 }, "id"),
                        right: column(BASE, "editor_id"),
                    },
                    source: TableSource {
                        alias: TableAlias { position: 2 },
                        namespace: TableNamespace::Framework,
                        table: "editors",
                    },
                },
            ],
            limit: SelectLimit::Unlimited,
            ordering: Vec::new(),
        });

        assert_eq!(
            statement.text,
            "SELECT \"t0\".\"id\" AS \"c0\", \"t1\".\"name\" AS \"c1\" FROM \"articles\" AS \"t0\" \
             JOIN \"authors\" AS \"t1\" ON \"t1\".\"id\" = \"t0\".\"author_id\" \
             LEFT JOIN \"margaret\".\"editors\" AS \"t2\" ON \"t2\".\"id\" = \"t0\".\"editor_id\" \
             WHERE \"t0\".\"id\" = $1"
        );
    }

    #[test]
    fn renders_the_children_of_a_page_through_a_lateral_subquery() {
        let parents = TableAlias { position: 0 };
        let children = TableAlias { position: 1 };
        let lateral = TableAlias { position: 2 };
        let statement = render_select(&Select {
            columns: vec![
                Expression::UnnestColumn {
                    alias: parents,
                    position: 0,
                },
                Expression::SubqueryColumn {
                    alias: lateral,
                    position: 0,
                },
            ],
            filter: SelectFilter::Everything,
            from: FromItem::Unnest(UnnestSource {
                alias: parents,
                arrays: vec![
                    ArrayParameter {
                        element_type: ColumnType::Uuid,
                        values: SqlParameter::new(Vec::<String>::new()),
                    },
                    ArrayParameter {
                        element_type: ColumnType::Text,
                        values: SqlParameter::new(Vec::<String>::new()),
                    },
                ],
            }),
            joins: vec![Join::Lateral {
                alias: lateral,
                select: Box::new(Select {
                    columns: vec![column(children, "locale")],
                    filter: SelectFilter::Matching(Condition::Compare {
                        comparison: Comparison::Equal,
                        left: column(children, "article_id"),
                        right: Expression::UnnestColumn {
                            alias: parents,
                            position: 0,
                        },
                    }),
                    from: FromItem::Table(TableSource {
                        alias: children,
                        namespace: TableNamespace::Application,
                        table: "article_translations",
                    }),
                    joins: Vec::new(),
                    limit: SelectLimit::Rows(21),
                    ordering: vec![OrderTerm {
                        direction: Direction::Ascending,
                        expression: column(children, "locale"),
                    }],
                }),
            }],
            limit: SelectLimit::Unlimited,
            ordering: Vec::new(),
        });

        assert_eq!(
            statement.text,
            "SELECT \"t0\".\"k0\" AS \"c0\", \"t2\".\"c0\" AS \"c1\" \
             FROM unnest($1::UUID[], $2::TEXT[]) AS \"t0\"(\"k0\", \"k1\") \
             CROSS JOIN LATERAL (SELECT \"t1\".\"locale\" AS \"c0\" FROM \"article_translations\" AS \"t1\" \
             WHERE \"t1\".\"article_id\" = \"t0\".\"k0\" ORDER BY \"t1\".\"locale\" ASC LIMIT 21) AS \"t2\""
        );
        assert_eq!(statement.parameters.len(), 2);
    }

    #[test]
    fn renders_a_negated_existence_probe_beside_a_null_check() {
        let revocations = TableAlias { position: 1 };
        let statement = render_select(&Select {
            columns: vec![column(BASE, "token")],
            filter: SelectFilter::Matching(Condition::And {
                left: Box::new(Condition::IsNull(vec![column(BASE, "redeemed_by")])),
                right: Box::new(Condition::Not(Box::new(Condition::Exists(ExistsProbe {
                    condition: Box::new(Condition::And {
                        left: Box::new(Condition::Compare {
                            comparison: Comparison::Equal,
                            left: column(revocations, "family"),
                            right: column(BASE, "family_id"),
                        }),
                        right: Box::new(Condition::Compare {
                            comparison: Comparison::Greater,
                            left: column(revocations, "expires_at"),
                            right: Expression::Parameter(SqlParameter::new(9_i64)),
                        }),
                    }),
                    source: TableSource {
                        alias: revocations,
                        namespace: TableNamespace::Framework,
                        table: "refresh_family_revocations",
                    },
                })))),
            }),
            from: FromItem::Table(TableSource {
                alias: BASE,
                namespace: TableNamespace::Framework,
                table: "refresh_tokens",
            }),
            joins: Vec::new(),
            limit: SelectLimit::Unlimited,
            ordering: Vec::new(),
        });

        assert_eq!(
            statement.text,
            "SELECT \"t0\".\"token\" AS \"c0\" FROM \"margaret\".\"refresh_tokens\" AS \"t0\" \
             WHERE ((\"t0\".\"redeemed_by\") IS NULL AND NOT (EXISTS (SELECT 1 FROM \"margaret\".\"refresh_family_revocations\" AS \"t1\" \
             WHERE (\"t1\".\"family\" = \"t0\".\"family_id\" AND \"t1\".\"expires_at\" > $1))))"
        );
    }
}
