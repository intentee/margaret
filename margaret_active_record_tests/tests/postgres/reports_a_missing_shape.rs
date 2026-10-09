use uuid::Uuid;

use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article::Article;
use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn reports_a_missing_shape() {
    let started = started_with_models().await;

    assert_eq!(
        Article::query()
            .id
            .eq(Uuid::nil())
            .load::<ArticleWithAuthor, _>(started.database.as_ref())
            .await
            .expect("the absent article is loaded"),
        Lookup::Missing
    );
}
