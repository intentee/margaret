use margaret::framework::active_record::model::Model;
use margaret::framework::active_record::next_page::NextPage;
use margaret::framework::active_record::page::Page;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::message::Message;

async fn attempted(database: &Database) {
    if let Ok(Page {
        next: NextPage::Continues(cursor),
        ..
    }) = Message::query()
        .posted_at
        .ascending()
        .limit::<2>()
        .fetch(database)
        .await
    {
        let _ = Message::query()
            .id
            .ascending()
            .limit::<2>()
            .resume(cursor)
            .fetch(database)
            .await;
    }
}

fn main() {
    drop(attempted);
}
