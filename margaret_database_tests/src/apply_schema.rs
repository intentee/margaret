use margaret_database::database::Database;
use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;

/// # Panics
///
/// Panics when the schema cannot be applied to the database.
pub async fn apply_schema(database: &Database, schema: &Schema) {
    database
        .client()
        .await
        .expect("a connection is checked out")
        .batch_execute(&render_postgres(schema))
        .await
        .expect("the generated schema applies to Postgres");
}
