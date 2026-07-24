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
use margaret_model::on_delete::OnDelete;
use margaret_model::render_postgres::render_postgres;
use margaret_model::schema::Schema;
use margaret_model::table::Table;

use margaret_schema_postgres_tests::start_database::start_database;

fn uuid_primary_key(name: &str) -> Column {
    Column {
        column_type: ColumnType::Uuid,
        default: ColumnDefault::UuidV7,
        name: name.to_string(),
        nullable: false,
    }
}

fn nullable_uuid(name: &str) -> Column {
    Column {
        column_type: ColumnType::Uuid,
        default: ColumnDefault::NotSet,
        name: name.to_string(),
        nullable: true,
    }
}

fn text_column(name: &str) -> Column {
    Column {
        column_type: ColumnType::Text,
        default: ColumnDefault::NotSet,
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
                columns: vec![uuid_primary_key("id"), text_column("name")],
                foreign_keys: Vec::new(),
                indexes: Vec::new(),
                name: "owners".to_string(),
                primary_key: names(&["id"]),
                unique_constraints: Vec::new(),
            },
            Table {
                columns: vec![uuid_primary_key("id"), nullable_uuid("owner_id")],
                foreign_keys: vec![ForeignKey {
                    columns: names(&["owner_id"]),
                    name: "widgets_owner_fkey".to_string(),
                    on_delete: OnDelete::SetNull,
                    references_columns: names(&["id"]),
                    references_table: "owners".to_string(),
                }],
                indexes: Vec::new(),
                name: "widgets".to_string(),
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
async fn an_omitted_nullable_foreign_key_defaults_to_null() {
    let database = start_database().await;
    let pool = database.pool();

    apply(pool).await;

    let widget_id = Uuid::new_v4();

    query("INSERT INTO widgets (id) VALUES ($1)")
        .bind(widget_id)
        .execute(pool)
        .await
        .expect("a widget without an owner is inserted");

    let owner_id: Option<Uuid> = query_scalar("SELECT owner_id FROM widgets WHERE id = $1")
        .bind(widget_id)
        .fetch_one(pool)
        .await
        .expect("the widget owner is read back");

    assert!(owner_id.is_none());
}

#[tokio::test]
async fn deleting_an_owner_nulls_the_widget_foreign_key() {
    let database = start_database().await;
    let pool = database.pool();

    apply(pool).await;

    let owner_id = Uuid::new_v4();
    let widget_id = Uuid::new_v4();

    query("INSERT INTO owners (id, name) VALUES ($1, $2)")
        .bind(owner_id)
        .bind("Grace")
        .execute(pool)
        .await
        .expect("the owner is inserted");

    query("INSERT INTO widgets (id, owner_id) VALUES ($1, $2)")
        .bind(widget_id)
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("the widget is inserted");

    query("DELETE FROM owners WHERE id = $1")
        .bind(owner_id)
        .execute(pool)
        .await
        .expect("the owner is deleted");

    let stored_owner_id: Option<Uuid> = query_scalar("SELECT owner_id FROM widgets WHERE id = $1")
        .bind(widget_id)
        .fetch_one(pool)
        .await
        .expect("the widget still exists");

    assert!(stored_owner_id.is_none());
}
