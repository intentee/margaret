use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_author_author::draft::Draft;
use margaret_active_record_tests::models::author::Author;

use crate::postgres::at_epoch_seconds::at_epoch_seconds;

pub async fn created_author(database: &Database, name: &str) -> Author {
    Author::create(Draft {
        name: name.to_string(),
        joined_at: at_epoch_seconds(1_600_000_000),
        bio: None,
    })
    .run(database)
    .await
    .expect("the author is created")
}
