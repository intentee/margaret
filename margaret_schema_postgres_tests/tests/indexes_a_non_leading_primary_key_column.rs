use margaret_model::column::Column;
use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::index::Index;
use margaret_model::schema::Schema;
use margaret_model::table::Table;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::assert_table_matches::assert_table_matches;
use margaret_schema_postgres_tests::start_database::start_database;

fn text_column(name: &str) -> Column {
    Column {
        column_type: ColumnType::Text,
        default: ColumnDefault::NotSet,
        name: name.to_string(),
        nullable: false,
    }
}

#[tokio::test]
async fn postgres_indexes_a_non_leading_member_of_a_composite_primary_key() {
    let database = start_database().await;
    let pool = database.pool();

    let declared = Schema {
        tables: vec![Table {
            columns: vec![
                text_column("repository"),
                text_column("branch"),
                text_column("hash"),
            ],
            foreign_keys: Vec::new(),
            indexes: vec![Index {
                columns: vec!["hash".to_string()],
                name: "lore_locks_hash_index".to_string(),
            }],
            name: "lore_locks".to_string(),
            primary_key: vec![
                "repository".to_string(),
                "branch".to_string(),
                "hash".to_string(),
            ],
            unique_constraints: Vec::new(),
        }],
    };

    apply_schema(pool, &declared).await;

    for table in &declared.tables {
        assert_table_matches(pool, table).await;
    }
}
