use crate::column::Column;
use crate::column_default::ColumnDefault;
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
    fn renders_a_composite_primary_key() {
        let schema = Schema {
            tables: vec![Table {
                columns: vec![
                    column("left", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                    column("right", ColumnType::Uuid, false, ColumnDefault::UuidV7),
                ],
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
                    name: "first".to_string(),
                    primary_key: Vec::new(),
                },
                Table {
                    columns: vec![column("id", ColumnType::Text, false, ColumnDefault::NotSet)],
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
                name: "empty".to_string(),
                primary_key: Vec::new(),
            }],
        };

        assert_eq!(render_postgres(&schema), "CREATE TABLE \"empty\" ();");
    }
}
