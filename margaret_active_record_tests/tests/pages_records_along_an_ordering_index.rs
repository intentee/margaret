use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::next_page::NextPage;
use margaret::framework::active_record::page::Page;
use margaret_active_record_tests::models::message::Message;

use crate::posted_message::posted_message;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn pages_records_along_an_ordering_index() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    posted_message(database, "third", 30).await;
    posted_message(database, "first", 10).await;
    posted_message(database, "second", 20).await;

    let Page { next, records } = Message::query()
        .posted_at
        .ascending()
        .limit::<2>()
        .fetch(database)
        .await
        .expect("the first page is read");

    assert_eq!(
        records
            .into_iter()
            .map(|message| message.body)
            .collect::<Vec<String>>(),
        ["first", "second"]
    );
    assert!(matches!(next, NextPage::Continues(_)));
}
