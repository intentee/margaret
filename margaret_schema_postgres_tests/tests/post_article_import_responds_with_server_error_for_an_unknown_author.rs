use uuid::Uuid;

use margaret::framework::validation::validation_result::ValidationResult;
use margaret_example::forms::post_article_form::PostArticleForm;
use margaret_example::routes::public::post_article_import::PostArticleImport;
use margaret_schema_postgres_tests::apply_schema::apply_schema;
use margaret_schema_postgres_tests::build_article_store::build_article_store;
use margaret_schema_postgres_tests::start_database::start_database;

#[tokio::test]
async fn post_article_import_responds_with_server_error_for_an_unknown_author() {
    let database = start_database().await;
    apply_schema(database.pool()).await;
    let store = build_article_store(&database);
    let responder = PostArticleImport::create(store);
    let form = ValidationResult::Valid(PostArticleForm {
        title: "Title".to_string(),
        body: "Body".to_string(),
        author_id: Uuid::from_u128(999),
    });

    assert_eq!(responder.respond(form).await.status(), 500);
}
