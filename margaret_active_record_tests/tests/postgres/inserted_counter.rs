use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::counter::Counter;

pub async fn inserted_counter(database: &Database, name: &str, hits: i64) -> Counter {
    let counter = Counter {
        name: name.to_string(),
        hits,
    };

    counter
        .insert()
        .run(database)
        .await
        .expect("the counter is inserted");

    counter
}
