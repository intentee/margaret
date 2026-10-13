use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database) {
    let _ = Article::query()
        .id
        .eq(Uuid::nil())
        .update(database, |columns| columns.id.to(Uuid::nil()))
        .await;
}

fn main() {
    drop(attempted);
}
