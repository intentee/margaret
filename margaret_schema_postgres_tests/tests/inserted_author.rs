use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_schema_postgres_fixture::author::Author;

use crate::joined_at::joined_at;

pub async fn inserted_author(database: &Database, name: &str) -> Author {
    let author = Author {
        id: Uuid::new_v4(),
        name: name.to_string(),
        active: true,
        joined_at: joined_at(),
        bio: None,
    };

    author
        .insert()
        .run(database)
        .await
        .expect("the author is inserted");

    author
}
