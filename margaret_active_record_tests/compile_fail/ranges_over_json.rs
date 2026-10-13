use std::collections::BTreeMap;

use uuid::Uuid;

use margaret::framework::active_record::json::Json;
use margaret::framework::active_record::model::Model;
use margaret::framework::database::database::Database;
use margaret_active_record_tests::models::article::Article;

async fn attempted(database: &Database) {
    let _ = Article::query()
        .id
        .eq(Uuid::nil())
        .when(|article| article.tags.above(Json::new(BTreeMap::new())))
        .find(database)
        .await;
}

fn main() {
    drop(attempted);
}
