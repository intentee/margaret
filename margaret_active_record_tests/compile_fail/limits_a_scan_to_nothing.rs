use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::message::Message;

fn main() {
    drop(Message::query().posted_at.ascending().limit::<0>());
}
