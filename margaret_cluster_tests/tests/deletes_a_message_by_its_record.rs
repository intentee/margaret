use chrono::DateTime;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::deletion::Deletion;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_cluster_fixture::margaret::models::models_message_message::draft::Draft;
use margaret_cluster_fixture::margaret::schema::SCHEMA;
use margaret_cluster_fixture::models::message::Message;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn deletes_a_message_by_its_record() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started.database.as_ref();
    let message = Message::create(Draft {
        body: "hello".to_string(),
        posted_at: DateTime::UNIX_EPOCH,
    })
    .run(database)
    .await
    .expect("the message is posted");
    let id = message.id;

    assert!(matches!(
        message
            .delete(database)
            .await
            .expect("the message is deleted"),
        Deletion::Deleted
    ));
    assert!(matches!(
        Message::query()
            .id
            .eq(id)
            .find(database)
            .await
            .expect("the message is looked up"),
        Lookup::Missing
    ));
}
