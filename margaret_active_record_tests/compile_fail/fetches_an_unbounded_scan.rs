use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::message::Message;

async fn attempted(database: &Database) {
    let _ = Message::query().posted_at.ascending().fetch(database).await;
}

fn main() {
    drop(attempted);
}
