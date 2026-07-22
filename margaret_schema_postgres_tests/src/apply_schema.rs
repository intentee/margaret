use sqlx::AssertSqlSafe;
use sqlx::PgPool;
use sqlx::raw_sql;

use margaret_example::margaret::schema::schema;
use margaret_model::render_postgres::render_postgres;

pub async fn apply_schema(pool: &PgPool) {
    let ddl = render_postgres(&schema());

    raw_sql(AssertSqlSafe(ddl))
        .execute(pool)
        .await
        .expect("the generated schema applies to Postgres");
}
