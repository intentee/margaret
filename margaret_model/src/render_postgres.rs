use crate::column::Column;
use crate::column_default::ColumnDefault;
use crate::foreign_key::ForeignKey;
use crate::index::Index;
use crate::on_delete::OnDelete;
use crate::schema::Schema;
use crate::table::Table;
use crate::unique_constraint::UniqueConstraint;

fn quote_identifier(identifier: &str) -> String {
    format!("\"{identifier}\"")
}

fn quote_identifier_list(identifiers: &[String]) -> String {
    identifiers
        .iter()
        .map(|identifier| quote_identifier(identifier))
        .collect::<Vec<String>>()
        .join(", ")
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

    definition
}

fn render_foreign_key(foreign_key: &ForeignKey) -> String {
    let mut definition = format!(
        "FOREIGN KEY ({}) REFERENCES {} ({})",
        quote_identifier(&foreign_key.column),
        quote_identifier(&foreign_key.references_table),
        quote_identifier(&foreign_key.references_column)
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

fn render_index(table_name: &str, index: &Index) -> String {
    format!(
        "CREATE INDEX {} ON {} ({});",
        quote_identifier(&index.name),
        quote_identifier(table_name),
        quote_identifier_list(&index.columns)
    )
}

fn render_unique_constraint(unique_constraint: &UniqueConstraint) -> String {
    format!(
        "UNIQUE ({})",
        quote_identifier_list(&unique_constraint.columns)
    )
}

fn render_table(table: &Table) -> String {
    let name = quote_identifier(&table.name);
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

    let mut foreign_key_lines: Vec<String> =
        table.foreign_keys.iter().map(render_foreign_key).collect();

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
        .map(|index| render_index(&table.name, index))
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
pub fn render_postgres(schema: &Schema) -> String {
    schema
        .tables
        .iter()
        .map(render_table)
        .collect::<Vec<String>>()
        .join("\n\n")
}

#[cfg(test)]
mod tests {
    use crate::column::Column;
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
            column_type,
            default,
            name: name.to_string(),
            nullable,
        }
    }

    fn foreign_key(
        column: &str,
        references_column: &str,
        references_table: &str,
        on_delete: OnDelete,
    ) -> ForeignKey {
        ForeignKey {
            column: column.to_string(),
            on_delete,
            references_column: references_column.to_string(),
            references_table: references_table.to_string(),
        }
    }

    fn index(columns: &[&str], name: &str) -> Index {
        Index {
            columns: columns.iter().map(|column| column.to_string()).collect(),
            name: name.to_string(),
        }
    }

    fn unique_constraint(columns: &[&str]) -> UniqueConstraint {
        UniqueConstraint {
            columns: columns.iter().map(|column| column.to_string()).collect(),
        }
    }

    #[test]
    fn renders_columns_types_nullability_default_and_primary_key() {
        let schema = Schema {
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
    fn renders_a_composite_primary_key() {
        let schema = Schema {
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
            tables: vec![Table {
                columns: vec![
                    column("region", ColumnType::Text, false, ColumnDefault::NotSet),
                    column("number", ColumnType::BigInt, false, ColumnDefault::NotSet),
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![foreign_key(
                    "author_id",
                    "id",
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
            tables: vec![Table {
                columns: vec![
                    column("id", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("title", ColumnType::Text, false, ColumnDefault::NotSet),
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![foreign_key(
                    "author_id",
                    "id",
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
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    true,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    "author_id",
                    "id",
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
            tables: vec![Table {
                columns: vec![
                    column("author_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                    column("editor_id", ColumnType::Uuid, false, ColumnDefault::NotSet),
                ],
                foreign_keys: vec![
                    foreign_key("editor_id", "id", "users", OnDelete::NoAction),
                    foreign_key("author_id", "id", "users", OnDelete::NoAction),
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
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key("author_id", "id", "authors", OnDelete::Cascade)],
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
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    "author_id",
                    "id",
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
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    true,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key("author_id", "id", "authors", OnDelete::SetNull)],
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
            tables: vec![Table {
                columns: vec![column(
                    "author_id",
                    ColumnType::Uuid,
                    false,
                    ColumnDefault::NotSet,
                )],
                foreign_keys: vec![foreign_key(
                    "author_id",
                    "id",
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
}
