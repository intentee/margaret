use futures_util::TryStreamExt as _;

use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::message::Message;

use crate::at_epoch_seconds::at_epoch_seconds;
use crate::posted_message::posted_message;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn streams_the_records_of_a_range_in_batches() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    posted_message(database, "old", 5).await;
    posted_message(database, "first", 10).await;
    posted_message(database, "second", 20).await;
    posted_message(database, "third", 30).await;

    assert_eq!(
        Message::query()
            .posted_at
            .at_least(at_epoch_seconds(10))
            .ascending()
            .stream::<2, _>(database)
            .map_ok(|message| message.body)
            .try_collect::<Vec<String>>()
            .await
            .expect("the messages are streamed"),
        ["first", "second", "third"]
    );
}
