use uuid::Uuid;

use crate::postgres::created_author::created_author;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn creates_a_record_with_its_database_defaults() {
    let started = started_with_models().await;

    assert_ne!(
        created_author(&started.database, "Milo").await.id,
        Uuid::nil()
    );
}
