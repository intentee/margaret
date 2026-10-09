use std::collections::BTreeMap;
use std::collections::HashMap;

use tokio_postgres::Client;

use margaret_database_tests::apply_schema::apply_schema;
use margaret_database_tests::started_database::StartedDatabase;
use margaret_model::table::Table;
use margaret_schema_postgres_fixture::margaret::schema::schema;

#[derive(Debug, Eq, PartialEq)]
struct ForeignKeyDefinition {
    columns: Vec<String>,
    references_columns: Vec<String>,
    references_table: String,
}

async fn unique_column_sets(client: &Client, table: &str) -> Vec<Vec<String>> {
    let rows = client
        .query(
            "SELECT tc.constraint_name::text AS constraint_name, kcu.column_name::text AS column_name \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
           ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema \
         WHERE tc.constraint_type = 'UNIQUE' AND tc.table_schema = 'public' AND tc.table_name = $1",
            &[&table],
        )
        .await
        .expect("the unique constraints are introspected");

    let mut sets: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for row in rows {
        sets.entry(row.get("constraint_name"))
            .or_default()
            .push(row.get("column_name"));
    }

    sets.into_values()
        .map(|mut columns| {
            columns.sort();
            columns
        })
        .collect()
}

async fn assert_columns_match(client: &Client, table: &Table) {
    let observed: HashMap<String, bool> = client
        .query(
            "SELECT column_name::text, is_nullable::text = 'YES' AS nullable FROM information_schema.columns \
             WHERE table_schema = 'public' AND table_name = $1",
            &[&table.name],
        )
        .await
        .expect("the table columns are introspected")
        .into_iter()
        .map(|row| (row.get("column_name"), row.get("nullable")))
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

async fn assert_primary_key_matches(client: &Client, table: &Table) {
    let mut observed: Vec<String> = client
        .query(
            "SELECT kcu.column_name::text \
         FROM information_schema.table_constraints tc \
         JOIN information_schema.key_column_usage kcu \
           ON kcu.constraint_name = tc.constraint_name AND kcu.table_schema = tc.table_schema \
         WHERE tc.constraint_type = 'PRIMARY KEY' AND tc.table_schema = 'public' AND tc.table_name = $1",
            &[&table.name],
        )
        .await
        .expect("the primary key is introspected")
        .into_iter()
        .map(|row| row.get(0))
        .collect();

    observed.sort();

    let mut expected = table.primary_key.clone();
    expected.sort();

    assert_eq!(
        observed, expected,
        "primary key of table '{}' does not match",
        table.name
    );
}

async fn foreign_key_definitions(client: &Client, table: &str) -> Vec<ForeignKeyDefinition> {
    client
        .query(
            "SELECT array_agg(local_attribute.attname::text ORDER BY local_key.ordinality), \
                referenced.relname::text, \
                array_agg(referenced_attribute.attname::text ORDER BY referenced_key.ordinality) \
         FROM pg_constraint constraint_entry \
         JOIN pg_class local ON local.oid = constraint_entry.conrelid \
         JOIN pg_class referenced ON referenced.oid = constraint_entry.confrelid \
         JOIN pg_namespace namespace ON namespace.oid = local.relnamespace \
         CROSS JOIN LATERAL unnest(constraint_entry.conkey) \
             WITH ORDINALITY AS local_key(attnum, ordinality) \
         CROSS JOIN LATERAL unnest(constraint_entry.confkey) \
             WITH ORDINALITY AS referenced_key(attnum, ordinality) \
         JOIN pg_attribute local_attribute \
           ON local_attribute.attrelid = constraint_entry.conrelid \
          AND local_attribute.attnum = local_key.attnum \
         JOIN pg_attribute referenced_attribute \
           ON referenced_attribute.attrelid = constraint_entry.confrelid \
          AND referenced_attribute.attnum = referenced_key.attnum \
         WHERE constraint_entry.contype = 'f' AND namespace.nspname = 'public' \
           AND local.relname = $1 AND local_key.ordinality = referenced_key.ordinality \
         GROUP BY constraint_entry.oid, referenced.relname",
            &[&table],
        )
        .await
        .expect("the foreign keys are introspected")
        .into_iter()
        .map(|row| ForeignKeyDefinition {
            columns: row.get(0),
            references_columns: row.get(2),
            references_table: row.get(1),
        })
        .collect()
}

async fn assert_foreign_keys_match(client: &Client, table: &Table) {
    let observed = foreign_key_definitions(client, &table.name).await;

    for foreign_key in &table.foreign_keys {
        let expected = ForeignKeyDefinition {
            columns: foreign_key.columns.clone(),
            references_columns: foreign_key.references_columns.clone(),
            references_table: foreign_key.references_table.clone(),
        };

        assert!(
            observed.contains(&expected),
            "foreign key over {:?} of table '{}' is missing",
            foreign_key.columns,
            table.name,
        );
    }
}

async fn assert_indexes_match(client: &Client, table: &Table) {
    for index in &table.indexes {
        let observed: Vec<String> = client
            .query(
                "SELECT a.attname::text \
             FROM pg_index i \
             JOIN pg_class ic ON ic.oid = i.indexrelid \
             JOIN pg_class tc ON tc.oid = i.indrelid \
             JOIN pg_namespace n ON n.oid = tc.relnamespace \
             JOIN LATERAL unnest(i.indkey::int2[]) WITH ORDINALITY AS k(attnum, ord) ON true \
             JOIN pg_attribute a ON a.attrelid = tc.oid AND a.attnum = k.attnum \
             WHERE n.nspname = 'public' AND tc.relname = $1 AND ic.relname = $2 \
             ORDER BY k.ord",
                &[&table.name, &index.name],
            )
            .await
            .expect("the index columns are introspected")
            .into_iter()
            .map(|row| row.get(0))
            .collect();

        assert_eq!(
            observed, index.columns,
            "index '{}' of table '{}' does not cover the expected columns in order",
            index.name, table.name
        );
    }
}

async fn assert_unique_constraints_exist(client: &Client, table: &Table) {
    for unique_constraint in &table.unique_constraints {
        let mut expected = unique_constraint.columns.clone();
        expected.sort();

        assert!(
            unique_column_sets(client, &table.name)
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
    let declared = schema();
    let started = StartedDatabase::start().await;

    apply_schema(&started.database, &declared).await;

    let client = started
        .database
        .client()
        .await
        .expect("a connection is checked out");

    for table in &declared.tables {
        assert_columns_match(&client, table).await;
        assert_primary_key_matches(&client, table).await;
        assert_foreign_keys_match(&client, table).await;
        assert_indexes_match(&client, table).await;
        assert_unique_constraints_exist(&client, table).await;
    }
}
