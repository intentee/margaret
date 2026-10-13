use margaret::framework::active_record::creatable::Creatable;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::margaret::models::models_message_message::draft::Draft;
use margaret_active_record_tests::models::message::Message;

use crate::postgres::at_epoch_seconds::at_epoch_seconds;

pub async fn posted_message(database: &Database, body: &str, posted_at: i64) -> Message {
    Message::create(Draft {
        body: body.to_string(),
        posted_at: at_epoch_seconds(posted_at),
    })
    .run(database)
    .await
    .expect("the message is posted")
}
