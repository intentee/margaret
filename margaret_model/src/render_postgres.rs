use crate::check_predicate::CheckPredicate;
use crate::column::Column;
use crate::column_check::ColumnCheck;
use crate::column_default::ColumnDefault;
use crate::foreign_key::ForeignKey;
use crate::framework_namespace::FRAMEWORK_NAMESPACE;
use crate::index::Index;
use crate::on_delete::OnDelete;
use crate::qualified_framework_table::qualified_framework_table;
use crate::schema::Schema;
use crate::table::Table;
use crate::unique_constraint::UniqueConstraint;

fn quote_identifier(identifier: &str) -> String {
    format!("\"{identifier}\"")
}

fn qualified_table(namespace: TableNamespace, table: &str) -> String {
    match namespace {
        TableNamespace::Application => quote_identifier(table),
        TableNamespace::Framework => qualified_framework_table(table),
    }
}

fn quote_identifier_list(identifiers: &[String]) -> String {
    identifiers
        .iter()
        .map(|identifier| quote_identifier(identifier))
        .collect::<Vec<String>>()
        .join(", ")
}

fn render_check_predicate(column_name: &str, predicate: CheckPredicate) -> String {
    match predicate {
        CheckPredicate::ByteLength { length } => {
            format!("length({}) = {length}", quote_identifier(column_name))
        }
        CheckPredicate::Minimum { minimum } => {
            format!("{} >= {minimum}", quote_identifier(column_name))
        }
    }
}

fn render_column_check(column_name: &str, check: &ColumnCheck) -> String {
    format!(
        "CONSTRAINT {} CHECK ({})",
        quote_identifier(&check.name),
        render_check_predicate(column_name, check.predicate)
    )
}

fn render_column(column: &Column) -> String {
    let mut definition = format!(
        "{} {}",
        quote_identifier(&column.name),
        column.column_type.render()
    );

    if !column.nullable {
        definition.push_str(" NOT NULL");
    }

    match column.default {
        ColumnDefault::NotSet => {}
        ColumnDefault::UuidV7 => definition.push_str(" DEFAULT uuidv7()"),
    }

    let mut check_definitions: Vec<String> = column
        .checks
        .iter()
        .map(|check| render_column_check(&column.name, check))
        .collect();

    check_definitions.sort();

    for check_definition in &check_definitions {
        definition.push(' ');
        definition.push_str(check_definition);
    }

    definition
}

fn render_foreign_key(namespace: TableNamespace, foreign_key: &ForeignKey) -> String {
    let mut definition = format!(
        "FOREIGN KEY ({}) REFERENCES {} ({})",
        quote_identifier_list(&foreign_key.columns),
        qualified_table(namespace, &foreign_key.references_table),
        quote_identifier_list(&foreign_key.references_columns)
    );

    match foreign_key.on_delete {
        OnDelete::Cascade => definition.push_str(" ON DELETE CASCADE"),
        OnDelete::NoAction => {}
        OnDelete::Restrict => definition.push_str(" ON DELETE RESTRICT"),
        OnDelete::SetDefault => definition.push_str(" ON DELETE SET DEFAULT"),
        OnDelete::SetNull => definition.push_str(" ON DELETE SET NULL"),
    }

    definition
}

fn render_index(namespace: TableNamespace, table_name: &str, index: &Index) -> String {
    format!(
        "CREATE INDEX {} ON {} ({});",
        quote_identifier(&index.name),
        qualified_table(namespace, table_name),
        quote_identifier_list(&index.columns)
    )
}

fn render_unique_constraint(unique_constraint: &UniqueConstraint) -> String {
    format!(
        "UNIQUE ({})",
        quote_identifier_list(&unique_constraint.columns)
    )
}

fn render_table(namespace: TableNamespace, table: &Table) -> String {
    let name = qualified_table(namespace, &table.name);
    let mut lines: Vec<String> = table.columns.iter().map(render_column).collect();

    if !table.primary_key.is_empty() {
        lines.push(format!(
            "PRIMARY KEY ({})",
            quote_identifier_list(&table.primary_key)
        ));
    }

    let mut unique_lines: Vec<String> = table
        .unique_constraints
        .iter()
        .map(render_unique_constraint)
        .collect();

    unique_lines.sort();
    lines.extend(unique_lines);

    let mut foreign_key_lines: Vec<String> = table
        .foreign_keys
        .iter()
        .map(|foreign_key| render_foreign_key(namespace, foreign_key))
        .collect();

    foreign_key_lines.sort();
    lines.extend(foreign_key_lines);

    let table_statement = if lines.is_empty() {
        format!("CREATE TABLE {name} ();")
    } else {
        let body = lines
            .iter()
            .map(|line| format!("    {line}"))
            .collect::<Vec<String>>()
            .join(",\n");

        format!("CREATE TABLE {name} (\n{body}\n);")
    };

    let mut index_statements: Vec<String> = table
        .indexes
        .iter()
        .map(|index| render_index(namespace, &table.name, index))
        .collect();

    index_statements.sort();

    if index_statements.is_empty() {
        return table_statement;
    }

    let mut statements = vec![table_statement];
    statements.extend(index_statements);

    statements.join("\n\n")
}

#[derive(Clone, Copy)]
enum TableNamespace {
    Application,
    Framework,
}

#[must_use]
pub fn render_postgres(
    Schema {
        framework_tables,
        tables,
    }: &Schema,
) -> String {
    let framework_namespace = (!framework_tables.is_empty())
        .then(|| format!("CREATE SCHEMA {};", quote_identifier(FRAMEWORK_NAMESPACE)));

    framework_namespace
        .into_iter()
        .chain(
            framework_tables
                .iter()
                .map(|table| render_table(TableNamespace::Framework, table)),
        )
        .chain(
            tables
                .iter()
                .map(|table| render_table(TableNamespace::Application, table)),
        )
        .collect::<Vec<String>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use crate::check_predicate::CheckPredicate;
    use crate::column::Column;
    use crate::column_check::ColumnCheck;
    use crate::column_default::ColumnDefault;
    use crate::column_type::ColumnType;
    use crate::foreign_key::ForeignKey;
    use crate::index::Index;
    use crate::on_delete::OnDelete;
    use crate::render_postgres::render_postgres;
    use crate::schema::Schema;
    use crate::table::Table;
    use crate::unique_constraint::UniqueConstraint;

    fn column(
        name: &str,
        column_type: ColumnType,
        nullable: bool,
        default: ColumnDefault,
    ) -> Column {
        Column {
            checks: Vec::new(),
            column_type,
            default,
            name: name.to_string(),
            nullable,
        }
    }

    fn checked_column(name: &str, column_type: ColumnType, checks: Vec<ColumnCheck>) -> Column {
        Column {
            checks,
            column_type,
            default: ColumnDefault::NotSet,
            name: name.to_string(),
            nullable: false,
        }
    }

    fn column_check(name: &str, predicate: CheckPredicate) -> ColumnCheck {
        ColumnCheck {
            name: name.to_string(),
            predicate,
        }
    }

    fn foreign_key(
        columns: &[&str],
        references_columns: &[&str],
        references_table: &str,
        on_delete: OnDelete,
    ) -> ForeignKey {
        ForeignKey {
            columns: columns.iter().map(ToString::to_string).collect(),
            on_delete,
            references_columns: references_columns.iter().map(ToString::to_string).collect(),
            references_table: references_table.to_string(),
        }
    }

    fn index(columns: &[&str], name: &str) -> Index {
        Index {
            columns: columns.iter().map(ToString::to_string).collect(),
            name: name.to_string(),
        }
    }

    fn unique_constraint(columns: &[&str]) -> UniqueConstraint {
        UniqueConstraint {
            columns: columns.iter().map(ToString::to_string).collect(),
        }
    }

    #[test]
    fn renders_framework_tables_in_the_framework_namespace_before_application_tables() {
        let schema = Schema {
            framework_tables: vec![Table {
                columns: vec![column(
                    "family",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["family"],
                    &["family"],
                    "families",
                    OnDelete::Cascade,
                )],
                indexes: vec![index(&["family"], "tokens_family")],
                name: "tokens".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
            tables: vec![Table {
                columns: vec![column("id", ColumnType::Uuid, false, ColumnDefault::NotSet)],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "things".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE SCHEMA \"margaret\";\n\nCREATE TABLE \"margaret\".\"tokens\" (\n    \"family\" UUID NOT NULL,\n    FOREIGN KEY (\"family\") REFERENCES \"margaret\".\"families\" (\"family\") ON DELETE CASCADE\n);\n\nCREATE INDEX \"tokens_family\" ON \"margaret\".\"tokens\" (\"family\");\n\nCREATE TABLE \"things\" (\n    \"id\" UUID NOT NULL\n);"
        );
    }

    #[test]
    fn renders_columns_types_nullability_default_and_primary_key() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("count", ColumnType::Integer, false, ColumnDefault::NotSet),
                    column("big", ColumnType::BigInt, false, ColumnDefault::NotSet),
                    column("flag", ColumnType::Boolean, false, ColumnDefault::NotSet),
                    column("label", ColumnType::Text, false, ColumnDefault::NotSet),
                    column("note", ColumnType::Text, true, ColumnDefault::NotSet),
                ],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "things".to_string(),
                primary_key: vec!["id".to_string()],
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"things\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"count\" INTEGER NOT NULL,\n    \"big\" BIGINT NOT NULL,\n    \"flag\" BOOLEAN NOT NULL,\n    \"label\" TEXT NOT NULL,\n    \"note\" TEXT,\n    PRIMARY KEY (\"id\")\n);"
        );
    }

    #[test]
    fn renders_a_timestamptz_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "created_at",
                    ColumnType::Timestamptz,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "events".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"events\" (\n    \"created_at\" TIMESTAMPTZ NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_bytea_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "data",
                    ColumnType::Bytea,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "files".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"files\" (\n    \"data\" BYTEA NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_real_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "reputation",
                    ColumnType::Real,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "authors".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"authors\" (\n    \"reputation\" REAL NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_double_precision_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "reading_minutes",
                    ColumnType::DoublePrecision,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "articles".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"articles\" (\n    \"reading_minutes\" DOUBLE PRECISION NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_numeric_column_with_its_precision_and_scale() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "price",
                    ColumnType::Numeric {
                        precision: 12,
                        scale: 2,
                    },
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "line_items".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"line_items\" (\n    \"price\" NUMERIC(12, 2) NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_nullable_numeric_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "discount",
                    ColumnType::Numeric {
                        precision: 5,
                        scale: 4,
                    },
                    true,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "line_items".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"line_items\" (\n    \"discount\" NUMERIC(5, 4)\n);"
        );
    }

    #[test]
    fn renders_a_composite_primary_key() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("left", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("right", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                ],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "pairs".to_string(),
                primary_key: vec!["left".to_string(), "right".to_string()],
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"pairs\" (\n    \"left\" UUID NOT NULL DEFAULT uuidv7(),\n    \"right\" UUID NOT NULL DEFAULT uuidv7(),\n    PRIMARY KEY (\"left\", \"right\")\n);"
        );
    }

    #[test]
    fn renders_a_single_column_unique_constraint() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("email", ColumnType::Text, false, ColumnDefault::NotSet),
                ],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "users".to_string(),
                primary_key: vec!["id".to_string()],
                unique_constraints: vec![unique_constraint(&["email"])],
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"users\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"email\" TEXT NOT NULL,\n    PRIMARY KEY (\"id\"),\n    UNIQUE (\"email\")\n);"
        );
    }

    #[test]
    fn renders_a_composite_unique_constraint_before_foreign_keys() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("region", ColumnType::Text, false, ColumnDefault::NotSet),
                    column("number", ColumnType::BigInt, false, ColumnDefault::NotSet),
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::NoAction,
                )],
                indexes: Vec::new(),
                name: "line_items".to_string(),
                primary_key: Vec::new(),
                unique_constraints: vec![unique_constraint(&["region", "number"])],
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"line_items\" (\n    \"region\" TEXT NOT NULL,\n    \"number\" BIGINT NOT NULL,\n    \"author_id\" UUID NOT NULL,\n    UNIQUE (\"region\", \"number\"),\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_multiple_tables_separated_by_a_blank_line() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![
                Table {
                    columns: vec![column("id", ColumnType::Text, false, ColumnDefault::NotSet)],
                    foreign_keys: Vec::new(),
                    indexes: Vec::new(),
                    name: "first".to_string(),
                    primary_key: Vec::new(),
                    unique_constraints: Vec::new(),
                },
                Table {
                    columns: vec![column("id", ColumnType::Text, false, ColumnDefault::NotSet)],
                    foreign_keys: Vec::new(),
                    indexes: Vec::new(),
                    name: "second".to_string(),
                    primary_key: Vec::new(),
                    unique_constraints: Vec::new(),
                },
            ],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"first\" (\n    \"id\" TEXT NOT NULL\n);\n\nCREATE TABLE \"second\" (\n    \"id\" TEXT NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_table_without_columns() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: Vec::new(),
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "empty".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(render_postgres(&schema), "CREATE TABLE \"empty\" ();");
    }

    #[test]
    fn renders_a_single_column_foreign_key() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("title", ColumnType::Text, false, ColumnDefault::NotSet),
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::NoAction,
                )],
                indexes: Vec::new(),
                name: "articles".to_string(),
                primary_key: vec!["id".to_string()],
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"articles\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"title\" TEXT NOT NULL,\n    \"author_id\" UUID NOT NULL,\n    PRIMARY KEY (\"id\"),\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_a_nullable_foreign_key_column() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    true,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::NoAction,
                )],
                indexes: Vec::new(),
                name: "posts".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_foreign_keys_in_a_deterministic_order() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                    column("editor_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![
                    foreign_key(&["editor_id"], &["id"], "users", OnDelete::NoAction),
                    foreign_key(&["author_id"], &["id"], "users", OnDelete::NoAction),
                ],
                indexes: Vec::new(),
                name: "docs".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"docs\" (\n    \"author_id\" UUID NOT NULL,\n    \"editor_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"users\" (\"id\"),\n    FOREIGN KEY (\"editor_id\") REFERENCES \"users\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_on_delete_cascade() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::Cascade,
                )],
                indexes: Vec::new(),
                name: "posts".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE CASCADE\n);"
        );
    }

    #[test]
    fn renders_on_delete_restrict() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::Restrict,
                )],
                indexes: Vec::new(),
                name: "posts".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE RESTRICT\n);"
        );
    }

    #[test]
    fn renders_on_delete_set_null() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    true,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::SetNull,
                )],
                indexes: Vec::new(),
                name: "posts".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE SET NULL\n);"
        );
    }

    #[test]
    fn renders_on_delete_set_default() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    &["author_id"],
                    &["id"],
                    "authors",
                    OnDelete::SetDefault,
                )],
                indexes: Vec::new(),
                name: "posts".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE SET DEFAULT\n);"
        );
    }

    #[test]
    fn renders_a_single_column_index() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column(
                        "created_at",
                        ColumnType::Timestamptz,
                        false,
                        ColumnDefault::NotSet,
                    ),
                ],
                foreign_keys: Vec::new(),
                indexes: vec![index(&["created_at"], "articles_created_at_index")],
                name: "articles".to_string(),
                primary_key: vec!["id".to_string()],
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"articles\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"created_at\" TIMESTAMPTZ NOT NULL,\n    PRIMARY KEY (\"id\")\n);\n\nCREATE INDEX \"articles_created_at_index\" ON \"articles\" (\"created_at\");"
        );
    }

    #[test]
    fn renders_a_multi_column_index() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column(
                        "is_active",
                        ColumnType::Boolean,
                        false,
                        ColumnDefault::NotSet,
                    ),
                    column(
                        "joined_at",
                        ColumnType::Timestamptz,
                        false,
                        ColumnDefault::NotSet,
                    ),
                ],
                foreign_keys: Vec::new(),
                indexes: vec![index(&["is_active", "joined_at"], "authors_active_joined")],
                name: "authors".to_string(),
                primary_key: vec!["id".to_string()],
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"authors\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"is_active\" BOOLEAN NOT NULL,\n    \"joined_at\" TIMESTAMPTZ NOT NULL,\n    PRIMARY KEY (\"id\")\n);\n\nCREATE INDEX \"authors_active_joined\" ON \"authors\" (\"is_active\", \"joined_at\");"
        );
    }

    #[test]
    fn renders_indexes_in_a_deterministic_order() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                    column(
                        "created_at",
                        ColumnType::Timestamptz,
                        false,
                        ColumnDefault::NotSet,
                    ),
                ],
                foreign_keys: Vec::new(),
                indexes: vec![
                    index(&["created_at"], "articles_created_at_index"),
                    index(&["author_id"], "articles_author_id_index"),
                ],
                name: "articles".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"articles\" (\n    \"author_id\" UUID NOT NULL,\n    \"created_at\" TIMESTAMPTZ NOT NULL\n);\n\nCREATE INDEX \"articles_author_id_index\" ON \"articles\" (\"author_id\");\n\nCREATE INDEX \"articles_created_at_index\" ON \"articles\" (\"created_at\");"
        );
    }

    #[test]
    fn renders_a_byte_length_check() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![checked_column(
                    "hash",
                    ColumnType::Bytea,
                    vec![column_check(
                        "fragment_metadata_hash_byte_length",
                        CheckPredicate::ByteLength { length: 32 },
                    )],
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "fragment_metadata".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"fragment_metadata\" (\n    \"hash\" BYTEA NOT NULL CONSTRAINT \"fragment_metadata_hash_byte_length\" CHECK (length(\"hash\") = 32)\n);"
        );
    }

    #[test]
    fn renders_a_minimum_check() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![checked_column(
                    "size_payload",
                    ColumnType::BigInt,
                    vec![column_check(
                        "fragment_metadata_size_payload_minimum",
                        CheckPredicate::Minimum { minimum: 0 },
                    )],
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "fragment_metadata".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"fragment_metadata\" (\n    \"size_payload\" BIGINT NOT NULL CONSTRAINT \"fragment_metadata_size_payload_minimum\" CHECK (\"size_payload\" >= 0)\n);"
        );
    }

    #[test]
    fn renders_checks_on_one_column_in_a_deterministic_order() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![checked_column(
                    "hash",
                    ColumnType::Bytea,
                    vec![
                        column_check(
                            "hashes_hash_minimum",
                            CheckPredicate::Minimum { minimum: 1 },
                        ),
                        column_check(
                            "hashes_hash_byte_length",
                            CheckPredicate::ByteLength { length: 32 },
                        ),
                    ],
                )],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "hashes".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"hashes\" (\n    \"hash\" BYTEA NOT NULL CONSTRAINT \"hashes_hash_byte_length\" CHECK (length(\"hash\") = 32) CONSTRAINT \"hashes_hash_minimum\" CHECK (\"hash\" >= 1)\n);"
        );
    }

    #[test]
    fn renders_a_composite_foreign_key() {
        let schema = Schema {
            framework_tables: Vec::new(),
            tables: vec![Table {
                columns: vec![
                    column("partition", ColumnType::Uuid, false, ColumnDefault::NotSet),
                    column("hash", ColumnType::Bytea, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![foreign_key(
                    &["partition", "hash"],
                    &["partition", "hash"],
                    "fragment_metadata",
                    OnDelete::NoAction,
                )],
                indexes: Vec::new(),
                name: "fragment".to_string(),
                primary_key: Vec::new(),
                unique_constraints: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"fragment\" (\n    \"partition\" UUID NOT NULL,\n    \"hash\" BYTEA NOT NULL,\n    FOREIGN KEY (\"partition\", \"hash\") REFERENCES \"fragment_metadata\" (\"partition\", \"hash\")\n);"
        );
    }
}
