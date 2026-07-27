use sqlx::AssertSqlSafe;
use sqlx::PgPool;
use sqlx::raw_sql;

use margaret_model::render_postgres::render_postgres;

use crate::schema_fixture::schema_fixture;

pub async fn apply_schema(pool: &PgPool) {
    let ddl = render_postgres(&schema_fixture());

    raw_sql(AssertSqlSafe(ddl))
        .execute(pool)
        .await
        .expect("the generated schema applies to Postgres");
}
