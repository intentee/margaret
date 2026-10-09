use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::next_page::NextPage;
use margaret::framework::active_record::page::Page;
use margaret_active_record_tests::models::message::Message;

use crate::postgres::posted_message::posted_message;
use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn pages_records_in_descending_order() {
    let started = started_with_models().await;
    let database = started.database.as_ref();

    posted_message(database, "first", 10).await;
    posted_message(database, "second", 20).await;
    posted_message(database, "third", 30).await;

    let Page {
        next: NextPage::Continues(cursor),
        ..
    } = Message::query()
        .posted_at
        .descending()
        .limit::<1>()
        .fetch(database)
        .await
        .expect("the first page is read")
    else {
        panic!("the first page continues");
    };

    assert_eq!(
        Message::query()
            .posted_at
            .descending()
            .limit::<1>()
            .resume(cursor)
            .fetch(database)
            .await
            .expect("the second page is read")
            .records
            .into_iter()
            .map(|message| message.body)
            .collect::<Vec<String>>(),
        ["second"]
    );
}
