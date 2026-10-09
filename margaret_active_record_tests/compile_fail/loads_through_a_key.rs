use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database, article: Article) {
    let _ = article.author.load(database).await;
}

fn main() {
    drop(attempted);
}
