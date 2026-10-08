use margaret::framework::active_record::primary_key_binder::PrimaryKeyBinder;
use margaret_active_record_tests::models::article_with_author::ArticleWithAuthor;
use margaret_route_parameter_binding::http_route_parameter_binder::HttpRouteParameterBinder;
use margaret_route_parameter_binding::route_parameter_binding_outcome::RouteParameterBindingOutcome;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn binds_a_shape_by_the_primary_key_of_its_model() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;

    assert!(matches!(
        PrimaryKeyBinder::<ArticleWithAuthor>::new(database)
            .bind(article.id.to_string())
            .await
            .expect("the article is bound"),
        RouteParameterBindingOutcome::Bound(bound) if bound == ArticleWithAuthor { article, author }
    ));
}
