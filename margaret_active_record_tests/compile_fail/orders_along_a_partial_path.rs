use uuid::Uuid;

use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database) {
    let _ = Article::query()
        .author
        .eq(Key::new(Uuid::nil()))
        .title
        .ascending()
        .limit::<10>()
        .fetch(database)
        .await;
}

fn main() {
    drop(attempted);
}
