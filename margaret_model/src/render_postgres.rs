use crate::column::Column;
use crate::column_default::ColumnDefault;
use crate::foreign_key::ForeignKey;
use crate::schema::Schema;
use crate::table::Table;

fn quote_identifier(identifier: &str) -> String {
    format!("\"{identifier}\"")
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
    let columns: Vec<String> = foreign_key
        .columns
        .iter()
        .map(|column| quote_identifier(column))
        .collect();
    let references_columns: Vec<String> = foreign_key
        .references_columns
        .iter()
        .map(|column| quote_identifier(column))
        .collect();

    format!(
        "FOREIGN KEY ({}) REFERENCES {} ({})",
        columns.join(", "),
        quote_identifier(&foreign_key.references_table),
        references_columns.join(", ")
    )
}

fn render_table(table: &Table) -> String {
    let name = quote_identifier(&table.name);
    let mut lines: Vec<String> = table.columns.iter().map(render_column).collect();

    if !table.primary_key.is_empty() {
        let keys: Vec<String> = table
            .primary_key
            .iter()
            .map(|key| quote_identifier(key))
            .collect();

        lines.push(format!("PRIMARY KEY ({})", keys.join(", ")));
    }

    let mut foreign_key_lines: Vec<String> =
        table.foreign_keys.iter().map(render_foreign_key).collect();

    foreign_key_lines.sort();
    lines.extend(foreign_key_lines);

    if lines.is_empty() {
        return format!("CREATE TABLE {name} ();");
    }

    let body = lines
        .iter()
        .map(|line| format!("    {line}"))
        .collect::<Vec<String>>()
        .join(",\n");

    format!("CREATE TABLE {name} (\n{body}\n);")
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
    use crate::render_postgres::render_postgres;
    use crate::schema::Schema;
    use crate::table::Table;

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
        columns: &[&str],
        references_columns: &[&str],
        references_table: &str,
    ) -> ForeignKey {
        ForeignKey {
            columns: columns.iter().map(|column| column.to_string()).collect(),
            references_columns: references_columns
                .iter()
                .map(|column| column.to_string())
                .collect(),
            references_table: references_table.to_string(),
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
                name: "things".to_string(),
                primary_key: vec!["id".to_string()],
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
                name: "events".to_string(),
                primary_key: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"events\" (\n    \"created_at\" TIMESTAMPTZ NOT NULL\n);"
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
                name: "pairs".to_string(),
                primary_key: vec!["left".to_string(), "right".to_string()],
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"pairs\" (\n    \"left\" UUID NOT NULL DEFAULT uuidv7(),\n    \"right\" UUID NOT NULL DEFAULT uuidv7(),\n    PRIMARY KEY (\"left\", \"right\")\n);"
        );
    }

    #[test]
    fn renders_multiple_tables_separated_by_a_blank_line() {
        let schema = Schema {
            tables: vec![
                Table {
                    columns: vec![column("id", ColumnType::Text, false, ColumnDefault::NotSet)],
                    foreign_keys: Vec::new(),
                    name: "first".to_string(),
                    primary_key: Vec::new(),
                },
                Table {
                    columns: vec![column("id", ColumnType::Text, false, ColumnDefault::NotSet)],
                    foreign_keys: Vec::new(),
                    name: "second".to_string(),
                    primary_key: Vec::new(),
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
                name: "empty".to_string(),
                primary_key: Vec::new(),
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
                foreign_keys: vec![foreign_key(&["author_id"], &["id"], "authors")],
                name: "articles".to_string(),
                primary_key: vec!["id".to_string()],
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"articles\" (\n    \"id\" UUID NOT NULL DEFAULT uuidv7(),\n    \"title\" TEXT NOT NULL,\n    \"author_id\" UUID NOT NULL,\n    PRIMARY KEY (\"id\"),\n    FOREIGN KEY (\"author_id\") REFERENCES \"authors\" (\"id\")\n);"
        );
    }

    #[test]
    fn renders_a_composite_foreign_key() {
        let schema = Schema {
            tables: vec![Table {
                columns: vec![
                    column(
                        "order_region",
                        ColumnType::Text,
                        false,
                        ColumnDefault::NotSet,
                    ),
                    column(
                        "order_number",
                        ColumnType::BigInt,
                        false,
                        ColumnDefault::NotSet,
                    ),
                ],
                foreign_keys: vec![foreign_key(
                    &["order_region", "order_number"],
                    &["region", "number"],
                    "orders",
                )],
                name: "line_items".to_string(),
                primary_key: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"line_items\" (\n    \"order_region\" TEXT NOT NULL,\n    \"order_number\" BIGINT NOT NULL,\n    FOREIGN KEY (\"order_region\", \"order_number\") REFERENCES \"orders\" (\"region\", \"number\")\n);"
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
                foreign_keys: vec![foreign_key(&["author_id"], &["id"], "authors")],
                name: "posts".to_string(),
                primary_key: Vec::new(),
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
                    foreign_key(&["editor_id"], &["id"], "users"),
                    foreign_key(&["author_id"], &["id"], "users"),
                ],
                name: "docs".to_string(),
                primary_key: Vec::new(),
            }],
        };

        assert_eq!(
            render_postgres(&schema),
            "CREATE TABLE \"docs\" (\n    \"author_id\" UUID NOT NULL,\n    \"editor_id\" UUID NOT NULL,\n    FOREIGN KEY (\"author_id\") REFERENCES \"users\" (\"id\"),\n    FOREIGN KEY (\"editor_id\") REFERENCES \"users\" (\"id\")\n);"
        );
    }
}
