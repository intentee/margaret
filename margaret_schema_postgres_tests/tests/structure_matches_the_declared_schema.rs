use std::collections::BTreeMap;
use std::collections::HashMap;

use sqlx::PgPool;
use sqlx::query_as;
use sqlx::query_scalar;

use margaret_example::margaret::schema::schema;
use margaret_model::table::Table;

use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::start_database::start_database;

async fn unique_column_sets(pool: &PgPool, table: &str) -> Vec<Vec<String>> {
    let rows: Vec<(String, String)> = query_as(
        "SELECT tc.constraint_name, kcu.column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
           ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema \
         WHERE tc.constraint_type = 'UNIQUE' AND tc.table_schema = 'public' AND tc.table_name = $1",
    )
    .bind(table)
    .fetch_all(pool)
    .await
    .expect("the unique constraints are introspected");

    let mut sets: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (constraint_name, column_name) in rows {
        sets.entry(constraint_name).or_default().push(column_name);
    }

    sets.into_values()
        .map(|mut columns| {
            columns.sort();
            columns
        })
        .collect()
}

async fn assert_columns_match(pool: &PgPool, table: &Table) {
    let observed: HashMap<String, bool> = query_as::<_, (String, String)>(
        "SELECT column_name, is_nullable FROM information_schema.columns \
         WHERE table_schema = 'public' AND table_name = $1",
    )
    .bind(&table.name)
    .fetch_all(pool)
    .await
    .expect("the table columns are introspected")
    .into_iter()
    .map(|(name, is_nullable)| (name, is_nullable == "YES"))
    .collect();

    for column in &table.columns {
        assert_eq!(
            observed.get(&column.name),
            Some(&column.nullable),
            "column '{}' of table '{}' is missing or has an unexpected nullability",
            column.name,
            table.name,
        );
    }
}

async fn assert_primary_key_matches(pool: &PgPool, table: &Table) {
    let mut observed: Vec<String> = query_scalar(
        "SELECT kcu.column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
           ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema \
         WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_schema = 'public' AND tc.table_name = $1",
    )
    .bind(&table.name)
    .fetch_all(pool)
    .await
    .expect("the primary key is introspected");

    observed.sort();

    let mut expected = table.primary_key.clone();
    expected.sort();

    assert_eq!(
        observed, expected,
        "primary key of table '{}' does not match",
        table.name
    );
}

async fn assert_foreign_keys_match(pool: &PgPool, table: &Table) {
    for foreign_key in &table.foreign_keys {
        let (referenced_table, referenced_column): (String, String) = query_as(
            "SELECT ccu.table_name, ccu.column_name \
             FROM information_schema.table_constraints tc \
             JOIN information_schema.key_column_usage kcu \
               ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema \
             JOIN information_schema.constraint_column_usage ccu \
               ON ccu.constraint_name = tc.constraint_name AND ccu.table_schema = tc.table_schema \
             WHERE tc.constraint_type = 'FOREIGN KEY' AND tc.table_schema = 'public' \
               AND tc.table_name = $1 AND kcu.column_name = $2",
        )
        .bind(&table.name)
        .bind(&foreign_key.column)
        .fetch_one(pool)
        .await
        .expect("the foreign key is introspected");

        assert_eq!(referenced_table, foreign_key.references_table);
        assert_eq!(referenced_column, foreign_key.references_column);
    }
}

async fn assert_indexes_exist(pool: &PgPool, table: &Table) {
    for index in &table.indexes {
        let count: i64 = query_scalar(
            "SELECT COUNT(*) FROM pg_indexes \
             WHERE schemaname = 'public' AND tablename = $1 AND indexname = $2",
        )
        .bind(&table.name)
        .bind(&index.name)
        .fetch_one(pool)
        .await
        .expect("the index is introspected");

        assert_eq!(
            count, 1,
            "index '{}' of table '{}' is missing",
            index.name, table.name
        );
    }
}

async fn assert_unique_constraints_exist(pool: &PgPool, table: &Table) {
    for unique_constraint in &table.unique_constraints {
        let mut expected = unique_constraint.columns.clone();
        expected.sort();

        assert!(
            unique_column_sets(pool, &table.name)
                .await
                .contains(&expected),
            "unique constraint over {:?} of table '{}' is missing",
            unique_constraint.columns,
            table.name,
        );
    }
}

#[tokio::test]
async fn the_applied_schema_matches_the_declared_structure() {
    let database = start_database().await;
    let pool = database.pool();

    apply_schema(pool).await;

    let declared = schema();

    for table in &declared.tables {
        assert_columns_match(pool, table).await;
        assert_primary_key_matches(pool, table).await;
        assert_foreign_keys_match(pool, table).await;
        assert_indexes_exist(pool, table).await;
        assert_unique_constraints_exist(pool, table).await;
    }
}
