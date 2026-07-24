use sqlx::AssertSqlSafe;
use sqlx::PgPool;
use sqlx::query;
use sqlx::query_scalar;
use sqlx::raw_sql;
use uuid::Uuid;

use margaret_model::column::Column;
use margaret_model::column_default::ColumnDefault;
use margaret_model::column_type::ColumnType;
use margaret_model::foreign_key::ForeignKey;
use margaret_model::index::Index;
use margaret_model::on_delete::OnDelete;
use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;
use margaret_model::table::Table;
use margaret_model::unique_constraint::UniqueConstraint;

use margaret_schema_postgres_tests::start_database::start_database;

fn text_column(name: &str) -> Column {
    Column {
        column_type: ColumnType::Text,
        default: ColumnDefault::NotSet,
        name: name.to_string(),
        nullable: false,
    }
}

fn uuid_primary_key(name: &str) -> Column {
    Column {
        column_type: ColumnType::Uuid,
        default: ColumnDefault::UuidV7,
        name: name.to_string(),
        nullable: false,
    }
}

fn names(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn schema() -> Schema {
    Schema {
        tables: vec![
            Table {
                columns: vec![
                    text_column("repository"),
                    text_column("branch"),
                    text_column("hash"),
                ],
                foreign_keys: Vec::new(),
                indexes: vec![Index {
                    columns: names(&["hash"]),
                    name: "lore_lock_hash_index".to_string(),
                }],
                name: "lore_lock".to_string(),
                primary_key: names(&["repository", "branch", "hash"]),
                unique_constraints: Vec::new(),
            },
            Table {
                columns: vec![
                    uuid_primary_key("id"),
                    text_column("lock_repository"),
                    text_column("lock_branch"),
                    text_column("lock_hash"),
                ],
                foreign_keys: vec![ForeignKey {
                    columns: names(&["lock_repository", "lock_branch", "lock_hash"]),
                    name: "holder_lock_fkey".to_string(),
                    on_delete: OnDelete::Cascade,
                    references_columns: names(&["repository", "branch", "hash"]),
                    references_table: "lore_lock".to_string(),
                }],
                indexes: Vec::new(),
                name: "lore_lock_holder".to_string(),
                primary_key: names(&["id"]),
                unique_constraints: Vec::new(),
            },
            Table {
                columns: vec![uuid_primary_key("id"), text_column("email")],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "profile_authors".to_string(),
                primary_key: names(&["id"]),
                unique_constraints: vec![UniqueConstraint {
                    columns: names(&["email"]),
                }],
            },
            Table {
                columns: vec![uuid_primary_key("id"), text_column("author_email")],
                foreign_keys: vec![ForeignKey {
                    columns: names(&["author_email"]),
                    name: "profiles_author_fkey".to_string(),
                    on_delete: OnDelete::NoAction,
                    references_columns: names(&["email"]),
                    references_table: "profile_authors".to_string(),
                }],
                indexes: Vec::new(),
                name: "profiles".to_string(),
                primary_key: names(&["id"]),
                unique_constraints: Vec::new(),
            },
        ],
    }
}

async fn apply(pool: &PgPool) {
    let ddl = render_postgres(&schema());

    raw_sql(AssertSqlSafe(ddl))
        .execute(pool)
        .await
        .expect("the generated schema applies to Postgres");
}

#[tokio::test]
async fn a_non_leading_primary_key_column_can_be_indexed_and_referenced() {
    let database = start_database().await;
    let pool = database.pool();

    apply(pool).await;

    let index_count: i64 = query_scalar(
        "SELECT count(*) FROM pg_indexes WHERE schemaname = 'public' \
         AND tablename = 'lore_lock' AND indexname = 'lore_lock_hash_index'",
    )
    .fetch_one(pool)
    .await
    .expect("the standalone hash index is introspected");

    assert_eq!(index_count, 1);

    query(
        "INSERT INTO lore_lock (repository, branch, hash) \
         VALUES ('intentee/margaret', 'main', 'abc123')",
    )
    .execute(pool)
    .await
    .expect("the lock row is inserted");

    let holder_referencing_present_lock = query(
        "INSERT INTO lore_lock_holder (id, lock_repository, lock_branch, lock_hash) \
         VALUES ($1, 'intentee/margaret', 'main', 'abc123')",
    )
    .bind(Uuid::new_v4())
    .execute(pool)
    .await;

    assert!(holder_referencing_present_lock.is_ok());

    let holder_referencing_absent_lock = query(
        "INSERT INTO lore_lock_holder (id, lock_repository, lock_branch, lock_hash) \
         VALUES ($1, 'intentee/margaret', 'main', 'not-a-hash')",
    )
    .bind(Uuid::new_v4())
    .execute(pool)
    .await;

    assert!(holder_referencing_absent_lock.is_err());

    query("INSERT INTO profile_authors (id, email) VALUES ($1, 'ada@example.com')")
        .bind(Uuid::new_v4())
        .execute(pool)
        .await
        .expect("the author row is inserted");

    let profile_referencing_present_email =
        query("INSERT INTO profiles (id, author_email) VALUES ($1, 'ada@example.com')")
            .bind(Uuid::new_v4())
            .execute(pool)
            .await;

    assert!(profile_referencing_present_email.is_ok());

    let profile_referencing_absent_email =
        query("INSERT INTO profiles (id, author_email) VALUES ($1, 'nobody@example.com')")
            .bind(Uuid::new_v4())
            .execute(pool)
            .await;

    assert!(profile_referencing_absent_email.is_err());
}
