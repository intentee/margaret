use chrono::DateTime;
use chrono::TimeDelta;

use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::active_record::model::Model;
use margaret_cluster_fixture::margaret::models::models_message_message::draft::Draft;
use margaret_cluster_fixture::margaret::schema::SCHEMA;
use margaret_cluster_fixture::models::message::Message;
use margaret_database_tests::started_database::StartedDatabase;

#[tokio::test]
async fn deletes_the_messages_posted_at_a_moment() {
    let started = StartedDatabase::with_schema(&SCHEMA).await;
    let database = started.database.as_ref();

    for (body, posted_at) in [
        ("first", DateTime::UNIX_EPOCH),
        ("second", DateTime::UNIX_EPOCH + TimeDelta::seconds(1)),
    ] {
        Message::create(Draft {
            body: body.to_string(),
            posted_at,
        })
        .run(database)
        .await
        .expect("the message is posted");
    }

    assert_eq!(
        Message::query()
            .posted_at
            .eq(DateTime::UNIX_EPOCH)
            .delete(database)
            .await
            .expect("the messages of the moment are deleted"),
        1
    );
    assert_eq!(
        Message::in_posting_order(database)
            .await
            .expect("the messages are read")
            .into_iter()
            .map(|Message { body, .. }| body)
            .collect::<Vec<String>>(),
        ["second"]
    );
}
