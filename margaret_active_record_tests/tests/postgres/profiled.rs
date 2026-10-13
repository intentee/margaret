use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::author::Author;
use margaret_active_record_tests::models::profile::Profile;

pub async fn profiled(database: &Database, author: &Author) -> Profile {
    let profile = Profile {
        author: Key::of(author),
        website: "https://example.com".to_string(),
    };

    profile
        .insert()
        .run(database)
        .await
        .expect("the profile is inserted");

    profile
}
