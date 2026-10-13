use uuid::Uuid;

use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_status::ArticleStatus;

async fn attempted(database: &Database) {
    let _ = Article::query()
        .id
        .eq(Uuid::nil())
        .when(|article| article.status.above(ArticleStatus::Draft))
        .find(database)
        .await;
}

fn main() {
    drop(attempted);
}
