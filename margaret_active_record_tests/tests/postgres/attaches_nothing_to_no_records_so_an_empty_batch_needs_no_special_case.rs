use margaret::framework::active_record::shape::Shape;
use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;

use crate::postgres::started_with_models::started_with_models;

#[tokio::test]
async fn attaches_nothing_to_no_records_so_an_empty_batch_needs_no_special_case() {
    let started = started_with_models().await;

    assert!(
        ArticleWithAuthor::attach(started.database.as_ref(), &[])
            .await
            .expect("nothing is attached")
            .is_empty()
    );
}
