use sqlx::AssertSqlSafe;
use sqlx::PgPool;
use sqlx::raw_sql;

use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;

pub async fn apply_schema(pool: &PgPool, schema: &Schema) {
    raw_sql(AssertSqlSafe(render_postgres(schema)))
        .execute(pool)
        .await
        .expect("the generated schema applies to Postgres");
}
