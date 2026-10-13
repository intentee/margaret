use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database) {
    let _ = Article::query().title.eq(String::new()).find(database).await;
}

fn main() {
    drop(attempted);
}
