use margaret::framework::active_record::key::Key;
use margaret::framework::active_record::lookup::Lookup;
use margaret::framework::active_record::model::Model;
use margaret_active_record_tests::models::article_translation::ArticleTranslation;

use crate::created_article::created_article;
use crate::created_author::created_author;
use crate::started_with_models::started_with_models;

#[tokio::test]
async fn finds_a_record_by_a_composite_primary_key() {
    let started = started_with_models().await;
    let database = started.database.as_ref();
    let author = created_author(database, "Milo").await;
    let article = created_article(database, &author, "Shipping").await;
    let translation = ArticleTranslation {
        article: Key::of(&article),
        locale: "pl".to_string(),
        title: "Wysyłka".to_string(),
    };

    translation
        .insert()
        .run(database)
        .await
        .expect("the translation is inserted");

    assert_eq!(
        ArticleTranslation::query()
            .article
            .eq(Key::of(&article))
            .locale
            .eq("pl".to_string())
            .find(database)
            .await
            .expect("the translation is read"),
        Lookup::Found(translation)
    );
}
