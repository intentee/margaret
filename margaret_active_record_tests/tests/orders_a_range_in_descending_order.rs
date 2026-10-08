use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::message::Message;

use crate::at_epoch_seconds::at_epoch_seconds;
use crate::posted_message::posted_message;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn orders_a_range_in_descending_order() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    posted_message(database, "first", 10).await;
    posted_message(database, "second", 20).await;
    posted_message(database, "third", 30).await;

    assert_eq!(
        Message::query()
            .posted_at
            .above(at_epoch_seconds(10))
            .descending()
            .limit::<5>()
            .fetch(database)
            .await
            .expect("the messages are read")
            .records
            .into_iter()
            .map(|message| message.body)
            .collect::<Vec<String>>(),
        ["third", "second"]
    );
}
