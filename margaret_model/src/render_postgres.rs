use margaret_sql_identifier::framework_namespace::FRAMEWORK_NAMESPACE;
use margaret_sql_identifier::qualified_table::qualified_table;
use margaret_sql_identifier::quote_identifier::quote_identifier;
use margaret_sql_identifier::table_namespace::TableNamespace;

use crate::check_predicate::CheckPredicate;
use crate::column::Column;
use crate::column_check::ColumnCheck;
use crate::column_default::ColumnDefault;
use crate::foreign_key::ForeignKey;
use crate::index::Index;
use crate::on_delete::OnDelete;
use crate::schema::Schema;
use crate::table::Table;
use crate::unique_constraint::UniqueConstraint;

fn quote_identifier_list(identifiers: &[&str]) -> String {
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
        quote_identifier(check.name),
        render_check_predicate(column_name, check.predicate)
    )
}

fn render_column(column: &Column) -> String {
    let mut definition = format!(
        "{} {}",
        quote_identifier(column.name),
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
        .map(|check| render_column_check(column.name, check))
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
        quote_identifier_list(foreign_key.columns),
        qualified_table(namespace, foreign_key.references_table),
        quote_identifier_list(foreign_key.references_columns)
    );

    match foreign_key.on_delete {
        OnDelete::Cascade => definition.push_str(" ON DELETE CASCADE"),
        OnDelete::NoAction => {}
        OnDelete::Restrict => definition.push_str(" ON DELETE RESTRICT"),
        OnDelete::SetNull => definition.push_str(" ON DELETE SET NULL"),
    }

    definition
}

fn render_index(namespace: TableNamespace, table_name: &str, index: &Index) -> String {
    format!(
        "CREATE INDEX {} ON {} ({});",
        quote_identifier(index.name),
        qualified_table(namespace, table_name),
        quote_identifier_list(index.columns)
    )
}

fn render_unique_constraint(unique_constraint: &UniqueConstraint) -> String {
    format!(
        "UNIQUE ({})",
        quote_identifier_list(unique_constraint.columns)
    )
}

fn render_table(table: &Table) -> String {
    let namespace = table.namespace;
    let name = qualified_table(namespace, table.name);
    let mut lines: Vec<String> = table.columns.iter().map(render_column).collect();

    if !table.primary_key.is_empty() {
        lines.push(format!(
            "PRIMARY KEY ({})",
            quote_identifier_list(table.primary_key)
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
        .map(|index| render_index(namespace, table.name, index))
        .collect();

    index_statements.sort();

    if index_statements.is_empty() {
        return table_statement;
    }

    let mut statements = vec![table_statement];
    statements.extend(index_statements);

    statements.join("\n\n")
}

#[must_use]
pub fn render_postgres(Schema { table_sets }: &Schema) -> String {
    let tables: Vec<&Table> = table_sets
        .iter()
        .flat_map(|set| set.iter().copied())
        .collect();
    let framework_namespace = tables
        .iter()
        .any(|table| table.namespace == TableNamespace::Framework)
        .then(|| format!("CREATE SCHEMA {};", quote_identifier(FRAMEWORK_NAMESPACE)));

    framework_namespace
        .into_iter()
        .chain(tables.into_iter().map(render_table))
        .collect::<Vec<String>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use margaret_sql_identifier::table_namespace::TableNamespace;

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

    const AUTHOR_FOREIGN_KEY: ForeignKey = ForeignKey {
        columns: &["author_id"],
        on_delete: OnDelete::NoAction,
        references_columns: &["id"],
        references_table: "authors",
    };
    const COLUMN: Column = Column {
        checks: &[],
        column_type: ColumnType::Text,
        default: ColumnDefault::NotSet,
        name: "",
        nullable: false,
    };
    const TABLE: Table = Table {
        columns: &[],
        foreign_keys: &[],
        indexes: &[],
        name: "",
        namespace: TableNamespace::Application,
        primary_key: &[],
        unique_constraints: &[],
    };

    #[test]
    fn renders_framework_tables_in_the_framework_namespace_before_application_tables() {
        const TOKENS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "family",
                ..COLUMN
            }],
            foreign_keys: &[ForeignKey {
                columns: &["family"],
                on_delete: OnDelete::Cascade,
                references_columns: &["family"],
                references_table: "families",
            }],
            indexes: &[Index {
                columns: &["family"],
                name: "tokens_family",
            }],
            name: "tokens",
            namespace: TableNamespace::Framework,
            primary_key: &[],
            unique_constraints: &[],
        };
        const THINGS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "id",
                ..COLUMN
            }],
            name: "things",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&TOKENS], &[&THINGS]],
            }),
            "CREATE SCHEMA \"margaret\";\n\nCREATE TABLE \"margaret\".\"tokens\" (\n    \"family\" UUID NOT NULL,\n    FOREIGN KEY (\"family\") REFERENCES \"margaret\".\"families\" (\"family\") ON DELETE CASCADE\n);\n\nCREATE INDEX \"tokens_family\" ON \"margaret\".\"tokens\" (\"family\");\n\nCREATE TABLE \"things\" (\n    \"id\" UUID NOT NULL\n);"
        );
    }

    #[test]
    fn renders_columns_types_nullability_default_and_primary_key() {
        const THINGS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Integer,
                    name: "count",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::BigInt,
                    name: "big",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Boolean,
                    name: "flag",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Text,
                    name: "label",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Text,
                    name: "note",
                    nullable: true,
                    ..COLUMN
                },
            ],
            name: "things",
            primary_key: &["id"],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&THINGS]],
            }),
            "CREATE TABLE \"things\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"count\" INTEGER NOT NULL,\n    \"big\" BIGINT NOT NULL,\n    \"flag\" BOOLEAN NOT NULL,\n    \"label\" TEXT NOT NULL,\n    \"note\" TEXT,\n    PRIMARY KEY (\"id\")\n);"
        );
    }

    #[test]
    fn renders_a_timestamptz_column() {
        const EVENTS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Timestamptz,
                name: "created_at",
                ..COLUMN
            }],
            name: "events",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&EVENTS]],
            }),
            "CREATE TABLE \"events\" (\n    \"created_at\" TIMESTAMPTZ NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_bytea_column() {
        const FILES: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Bytea,
                name: "data",
                ..COLUMN
            }],
            name: "files",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&FILES]],
            }),
            "CREATE TABLE \"files\" (\n    \"data\" BYTEA NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_real_column() {
        const AUTHORS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Real,
                name: "reputation",
                ..COLUMN
            }],
            name: "authors",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&AUTHORS]],
            }),
            "CREATE TABLE \"authors\" (\n    \"reputation\" REAL NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_double_precision_column() {
        const ARTICLES: Table = Table {
            columns: &[Column {
                column_type: ColumnType::DoublePrecision,
                name: "reading_minutes",
                ..COLUMN
            }],
            name: "articles",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&ARTICLES]],
            }),
            "CREATE TABLE \"articles\" (\n    \"reading_minutes\" DOUBLE PRECISION NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_numeric_column_with_its_precision_and_scale() {
        const LINE_ITEMS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Numeric {
                    precision: 12,
                    scale: 2,
                },
                name: "price",
                ..COLUMN
            }],
            name: "line_items",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&LINE_ITEMS]],
            }),
            "CREATE TABLE \"line_items\" (\n    \"price\" NUMERIC(12, 2) NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_nullable_numeric_column() {
        const LINE_ITEMS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Numeric {
                    precision: 5,
                    scale: 4,
                },
                name: "discount",
                nullable: true,
                ..COLUMN
            }],
            name: "line_items",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&LINE_ITEMS]],
            }),
            "CREATE TABLE \"line_items\" (\n    \"discount\" NUMERIC(5, 4)\n);"
        );
    }

    #[test]
    fn renders_a_composite_primary_key() {
        const PAIRS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "left",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "right",
                    ..COLUMN
                },
            ],
            name: "pairs",
            primary_key: &["left", "right"],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&PAIRS]],
            }),
            "CREATE TABLE \"pairs\" (\n    \"left\" UUID NOT NULL DEFAULT uuidv7(),\n    \"right\" UUID NOT NULL DEFAULT uuidv7(),\n    PRIMARY KEY (\"left\", \"right\")\n);"
        );
    }

    #[test]
    fn renders_a_single_column_unique_constraint() {
        const USERS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Text,
                    name: "email",
                    ..COLUMN
                },
            ],
            name: "users",
            primary_key: &["id"],
            unique_constraints: &[UniqueConstraint {
                columns: &["email"],
            }],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&USERS]],
            }),
            "CREATE TABLE \"users\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"email\" TEXT NOT NULL,\n    PRIMARY KEY (\"id\"),\n    UNIQUE (\"email\")\n);"
        );
    }

    #[test]
    fn renders_a_composite_unique_constraint_before_foreign_keys() {
        const LINE_ITEMS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Text,
                    name: "region",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::BigInt,
                    name: "number",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Uuid,
                    name: "author_id",
                    ..COLUMN
                },
            ],
            foreign_keys: &[AUTHOR_FOREIGN_KEY],
            name: "line_items",
            unique_constraints: &[UniqueConstraint {
                columns: &["region", "number"],
            }],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&LINE_ITEMS]],
            }),
            "CREATE TABLE \"line_items\" (\n    \"region\" TEXT NOT NULL,\n    \"number\" BIGINT NOT NULL,\n    \"author_id\" UUID NOT NULL,\n    UNIQUE (\"region\", \"number\"),\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_unique_constraints_in_a_deterministic_order() {
        const PAIRS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Text,
                    name: "left",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Text,
                    name: "right",
                    ..COLUMN
                },
            ],
            name: "pairs",
            unique_constraints: &[
                UniqueConstraint {
                    columns: &["right"],
                },
                UniqueConstraint { columns: &["left"] },
            ],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&PAIRS]],
            }),
            "CREATE TABLE \"pairs\" (\n    \"left\" TEXT NOT NULL,\n    \"right\" TEXT NOT NULL,\n    UNIQUE (\"left\"),\n    UNIQUE (\"right\")\n);"
        );
    }

    #[test]
    fn renders_multiple_tables_separated_by_a_blank_line() {
        const FIRST: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Text,
                name: "id",
                ..COLUMN
            }],
            name: "first",
            ..TABLE
        };
        const SECOND: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Text,
                name: "id",
                ..COLUMN
            }],
            name: "second",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&FIRST, &SECOND]],
            }),
            "CREATE TABLE \"first\" (\n    \"id\" TEXT NOT NULL\n);\n\nCREATE TABLE \"second\" (\n    \"id\" TEXT NOT NULL\n);"
        );
    }

    #[test]
    fn renders_a_table_without_columns() {
        const EMPTY: Table = Table {
            columns: &[],
            name: "empty",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&EMPTY]],
            }),
            "CREATE TABLE \"empty\" ();"
        );
    }

    #[test]
    fn renders_a_single_column_foreign_key() {
        const ARTICLES: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Text,
                    name: "title",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Uuid,
                    name: "author_id",
                    ..COLUMN
                },
            ],
            foreign_keys: &[AUTHOR_FOREIGN_KEY],
            name: "articles",
            primary_key: &["id"],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&ARTICLES]],
            }),
            "CREATE TABLE \"articles\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"title\" TEXT NOT NULL,\n    \"author_id\" UUID NOT NULL,\n    PRIMARY KEY (\"id\"),\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_a_nullable_foreign_key_column() {
        const POSTS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "author_id",
                nullable: true,
                ..COLUMN
            }],
            foreign_keys: &[AUTHOR_FOREIGN_KEY],
            name: "posts",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&POSTS]],
            }),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_foreign_keys_in_a_deterministic_order() {
        const DOCS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    name: "author_id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Uuid,
                    name: "editor_id",
                    ..COLUMN
                },
            ],
            foreign_keys: &[
                ForeignKey {
                    columns: &["editor_id"],
                    on_delete: OnDelete::NoAction,
                    references_columns: &["id"],
                    references_table: "users",
                },
                ForeignKey {
                    columns: &["author_id"],
                    on_delete: OnDelete::NoAction,
                    references_columns: &["id"],
                    references_table: "users",
                },
            ],
            name: "docs",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&DOCS]],
            }),
            "CREATE TABLE \"docs\" (\n    \"author_id\" UUID NOT NULL,\n    \"editor_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"users\" (\"id\"),\n    FOREIGN KEY (\"editor_id\") REFERENCES \"users\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_on_delete_cascade() {
        const POSTS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "author_id",
                ..COLUMN
            }],
            foreign_keys: &[ForeignKey {
                on_delete: OnDelete::Cascade,
                ..AUTHOR_FOREIGN_KEY
            }],
            name: "posts",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&POSTS]],
            }),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE CASCADE\n);"
        );
    }

    #[test]
    fn renders_on_delete_restrict() {
        const POSTS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "author_id",
                ..COLUMN
            }],
            foreign_keys: &[ForeignKey {
                on_delete: OnDelete::Restrict,
                ..AUTHOR_FOREIGN_KEY
            }],
            name: "posts",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&POSTS]],
            }),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE RESTRICT\n);"
        );
    }

    #[test]
    fn renders_on_delete_set_null() {
        const POSTS: Table = Table {
            columns: &[Column {
                column_type: ColumnType::Uuid,
                name: "author_id",
                nullable: true,
                ..COLUMN
            }],
            foreign_keys: &[ForeignKey {
                on_delete: OnDelete::SetNull,
                ..AUTHOR_FOREIGN_KEY
            }],
            name: "posts",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&POSTS]],
            }),
            "CREATE TABLE \"posts\" (\n    \"author_id\" UUID,\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\") ON DELETE SET NULL\n);"
        );
    }

    #[test]
    fn renders_a_single_column_index() {
        const ARTICLES: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Timestamptz,
                    name: "created_at",
                    ..COLUMN
                },
            ],
            indexes: &[Index {
                columns: &["created_at"],
                name: "articles_created_at_index",
            }],
            name: "articles",
            primary_key: &["id"],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&ARTICLES]],
            }),
            "CREATE TABLE \"articles\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"created_at\" TIMESTAMPTZ NOT NULL,\n    PRIMARY KEY (\"id\")\n);\n\nCREATE INDEX \"articles_created_at_index\" ON \"articles\" (\"created_at\");"
        );
    }

    #[test]
    fn renders_a_multi_column_index() {
        const AUTHORS: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    default: ColumnDefault::UuidV7,
                    name: "id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Boolean,
                    name: "is_active",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Timestamptz,
                    name: "joined_at",
                    ..COLUMN
                },
            ],
            indexes: &[Index {
                columns: &["is_active", "joined_at"],
                name: "authors_active_joined",
            }],
            name: "authors",
            primary_key: &["id"],
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&AUTHORS]],
            }),
            "CREATE TABLE \"authors\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"is_active\" BOOLEAN NOT NULL,\n    \"joined_at\" TIMESTAMPTZ NOT NULL,\n    PRIMARY KEY (\"id\")\n);\n\nCREATE INDEX \"authors_active_joined\" ON \"authors\" (\"is_active\", \"joined_at\");"
        );
    }

    #[test]
    fn renders_indexes_in_a_deterministic_order() {
        const ARTICLES: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    name: "author_id",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Timestamptz,
                    name: "created_at",
                    ..COLUMN
                },
            ],
            indexes: &[
                Index {
                    columns: &["created_at"],
                    name: "articles_created_at_index",
                },
                Index {
                    columns: &["author_id"],
                    name: "articles_author_id_index",
                },
            ],
            name: "articles",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&ARTICLES]],
            }),
            "CREATE TABLE \"articles\" (\n    \"author_id\" UUID NOT NULL,\n    \"created_at\" TIMESTAMPTZ NOT NULL\n);\n\nCREATE INDEX \"articles_author_id_index\" ON \"articles\" (\"author_id\");\n\nCREATE INDEX \"articles_created_at_index\" ON \"articles\" (\"created_at\");"
        );
    }

    #[test]
    fn renders_a_byte_length_check() {
        const FRAGMENT_METADATA: Table = Table {
            columns: &[Column {
                checks: &[ColumnCheck {
                    name: "fragment_metadata_hash_byte_length",
                    predicate: CheckPredicate::ByteLength { length: 32 },
                }],
                column_type: ColumnType::Bytea,
                name: "hash",
                ..COLUMN
            }],
            name: "fragment_metadata",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&FRAGMENT_METADATA]],
            }),
            "CREATE TABLE \"fragment_metadata\" (\n    \"hash\" BYTEA NOT NULL CONSTRAINT \"fragment_metadata_hash_byte_length\" CHECK (length(\"hash\") = 32)\n);"
        );
    }

    #[test]
    fn renders_a_minimum_check() {
        const FRAGMENT_METADATA: Table = Table {
            columns: &[Column {
                checks: &[ColumnCheck {
                    name: "fragment_metadata_size_payload_minimum",
                    predicate: CheckPredicate::Minimum { minimum: 0 },
                }],
                column_type: ColumnType::BigInt,
                name: "size_payload",
                ..COLUMN
            }],
            name: "fragment_metadata",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&FRAGMENT_METADATA]],
            }),
            "CREATE TABLE \"fragment_metadata\" (\n    \"size_payload\" BIGINT NOT NULL CONSTRAINT \"fragment_metadata_size_payload_minimum\" CHECK (\"size_payload\" >= 0)\n);"
        );
    }

    #[test]
    fn renders_checks_on_one_column_in_a_deterministic_order() {
        const HASHES: Table = Table {
            columns: &[Column {
                checks: &[
                    ColumnCheck {
                        name: "hashes_hash_minimum",
                        predicate: CheckPredicate::Minimum { minimum: 1 },
                    },
                    ColumnCheck {
                        name: "hashes_hash_byte_length",
                        predicate: CheckPredicate::ByteLength { length: 32 },
                    },
                ],
                column_type: ColumnType::Bytea,
                name: "hash",
                ..COLUMN
            }],
            name: "hashes",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&HASHES]],
            }),
            "CREATE TABLE \"hashes\" (\n    \"hash\" BYTEA NOT NULL CONSTRAINT \"hashes_hash_byte_length\" CHECK (length(\"hash\") = 32) CONSTRAINT \"hashes_hash_minimum\" CHECK (\"hash\" >= 1)\n);"
        );
    }

    #[test]
    fn renders_a_composite_foreign_key() {
        const FRAGMENT: Table = Table {
            columns: &[
                Column {
                    column_type: ColumnType::Uuid,
                    name: "partition",
                    ..COLUMN
                },
                Column {
                    column_type: ColumnType::Bytea,
                    name: "hash",
                    ..COLUMN
                },
            ],
            foreign_keys: &[ForeignKey {
                columns: &["partition", "hash"],
                on_delete: OnDelete::NoAction,
                references_columns: &["partition", "hash"],
                references_table: "fragment_metadata",
            }],
            name: "fragment",
            ..TABLE
        };

        assert_eq!(
            render_postgres(&Schema {
                table_sets: &[&[&FRAGMENT]],
            }),
            "CREATE TABLE \"fragment\" (\n    \"partition\" UUID NOT NULL,\n    \"hash\" BYTEA NOT NULL,\n    FOREIGN KEY (\"partition\", \"hash\") REFERENCES \"fragment_metadata\" (\"partition\", \"hash\")\n);"
        );
    }
}
