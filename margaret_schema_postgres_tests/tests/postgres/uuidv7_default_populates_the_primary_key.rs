use margaret::framework::active_record::creatable::Creatable;
use margaret_schema_postgres_fixture::author::Author;
use margaret_schema_postgres_fixture::margaret::models::author_author::draft::Draft;

use crate::postgres::joined_at::joined_at;
use crate::postgres::started_with_fixture::started_with_fixture;

#[tokio::test]
async fn the_uuidv7_default_populates_an_omitted_primary_key() {
    let started = started_with_fixture().await;

    let author = Author::create(Draft {
        name: "Grace Hopper".to_string(),
        active: true,
        joined_at: joined_at(),
        bio: None,
    })
    .run(started.database.as_ref())
    .await
    .expect("the author is created with a defaulted primary key");

    assert_eq!(author.id.get_version_num(), 7);
}
