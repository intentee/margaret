use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database) {
    let _ = Article::query()
        .author
        .above(Key::new(Uuid::nil()))
        .delete(database)
        .await;
}

fn main() {
    drop(attempted);
}
